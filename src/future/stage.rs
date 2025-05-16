use std::{
    future::Future,
    ops::ControlFlow,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[cfg(feature = "metrics")]
use std::time::Instant;

use http_body_util::{combinators::Collect, BodyExt};
use hyper::{
    body::{Bytes, Incoming},
    Response as HyperResponse, StatusCode,
};
use hyper_util::client::legacy::ResponseFuture as HyperResponseFuture;
use leaky_bucket::{AcquireOwned, RateLimiter};
use pin_project::pin_project;
use tokio::time::Timeout;

use crate::{
    client::OsuInner,
    error::{ApiError, OsuError},
    prelude::UserExtended,
    request::Request,
    OsuResult,
};

use super::request_generator::FutureRequestGenerator;

#[pin_project]
struct Ratelimit {
    #[pin]
    acquire: AcquireOwned,
    generator: Option<FutureRequestGenerator>,
}

impl Ratelimit {
    fn new(ratelimiter: Arc<RateLimiter>, generator: FutureRequestGenerator) -> Self {
        Self {
            acquire: ratelimiter.acquire_owned(1),
            generator: Some(generator),
        }
    }
}

impl Future for Ratelimit {
    type Output = FutureRequestGenerator;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.acquire.poll(cx) {
            Poll::Ready(_) => Poll::Ready(this.generator.take().expect("missing generator")),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[pin_project]
struct InFlight {
    #[pin]
    future: Timeout<HyperResponseFuture>,
    generator: Option<FutureRequestGenerator>,
    #[cfg(feature = "metrics")]
    start: Option<Instant>,
}

impl InFlight {
    fn new(future: HyperResponseFuture, generator: FutureRequestGenerator) -> Self {
        Self {
            future: tokio::time::timeout(generator.osu.timeout, future),
            generator: Some(generator),
            #[cfg(feature = "metrics")]
            start: None,
        }
    }
}

enum InFlightOutput {
    Ratelimit(Ratelimit),
    Chunking(Chunking),
    Failed(OsuError),
}

impl Future for InFlight {
    type Output = InFlightOutput;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        #[cfg(feature = "metrics")]
        let start = *this.start.get_or_insert_with(Instant::now);

        match this.future.poll(cx) {
            Poll::Ready(Ok(Ok(resp))) => {
                let status = resp.status();

                match status {
                    StatusCode::NOT_FOUND => {
                        return Poll::Ready(InFlightOutput::Failed(OsuError::NotFound))
                    }
                    StatusCode::SERVICE_UNAVAILABLE => {
                        return Poll::Ready(InFlightOutput::Failed(OsuError::ServiceUnavailable {
                            body: resp.into_body(),
                        }))
                    }
                    StatusCode::TOO_MANY_REQUESTS => warn!("429 response: {resp:?}"),
                    _ => {}
                }

                let generator = this.generator.take().expect("missing generator");

                let chunking = Chunking::new(
                    resp,
                    generator.osu,
                    #[cfg(feature = "metrics")]
                    ChunkingMetrics {
                        start,
                        route: generator.route,
                    },
                );

                Poll::Ready(InFlightOutput::Chunking(chunking))
            }
            Poll::Ready(Ok(Err(source))) => {
                Poll::Ready(InFlightOutput::Failed(OsuError::Request { source }))
            }
            Poll::Ready(Err(_)) => {
                let mut generator = this.generator.take().expect("missing generator");
                let max_retries = generator.osu.retries;

                if generator.attempt >= max_retries {
                    return Poll::Ready(InFlightOutput::Failed(OsuError::RequestTimeout));
                }

                generator.attempt += 1;

                warn!(
                    "Timed out on attempt {}/{max_retries}, retrying...",
                    generator.attempt
                );

                let ratelimiter = Arc::clone(&generator.osu.ratelimiter);

                Poll::Ready(InFlightOutput::Ratelimit(Ratelimit::new(
                    ratelimiter,
                    generator,
                )))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[pin_project]
pub(super) struct Chunking {
    #[pin]
    future: Collect<Incoming>,
    status: StatusCode,
    osu: Arc<OsuInner>,
    #[cfg(feature = "metrics")]
    metrics: ChunkingMetrics,
}

#[cfg(feature = "metrics")]
pub(super) struct ChunkingMetrics {
    pub(super) start: Instant,
    pub(super) route: &'static str,
}

impl Chunking {
    pub(super) fn new(
        resp: HyperResponse<Incoming>,
        osu: Arc<OsuInner>,
        #[cfg(feature = "metrics")] metrics: ChunkingMetrics,
    ) -> Self {
        Self {
            status: resp.status(),
            future: resp.into_body().collect(),
            osu,
            #[cfg(feature = "metrics")]
            metrics,
        }
    }
}

impl Future for Chunking {
    type Output = OsuResult<(Bytes, Arc<OsuInner>)>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let bytes = match this.future.poll(cx) {
            Poll::Ready(Ok(collected)) => collected.to_bytes(),
            Poll::Ready(Err(source)) => {
                return Poll::Ready(Err(OsuError::ChunkingResponse { source }));
            }
            Poll::Pending => return Poll::Pending,
        };

        #[cfg(feature = "metrics")]
        ::metrics::histogram!(crate::metrics::RESPONSE_TIME, "route" => this.metrics.route)
            .record(this.metrics.start.elapsed());

        // let text = String::from_utf8_lossy(&bytes);
        // println!("Response:\n{text}");

        let status = *this.status;
        let osu = Arc::clone(this.osu);

        if status.is_success() {
            return Poll::Ready(Ok((bytes, osu)));
        }

        let err = match serde_json::from_slice::<ApiError>(&bytes) {
            Ok(source) => OsuError::Response {
                bytes,
                source,
                status,
            },
            Err(source) => OsuError::Parsing { bytes, source },
        };

        Poll::Ready(Err(err))
    }
}

#[pin_project(project = StageInnerProject, project_replace = StageInnerReplace)]
#[allow(
    private_interfaces,
    reason = "we need this enum to be `pub(super)` solely to access its `new` \
    method; not to use its variants"
)]
pub(super) enum OsuRequestStageInner {
    Ratelimit(#[pin] Ratelimit),
    InFlight(#[pin] InFlight),
    Chunking(#[pin] Chunking),
}

impl OsuRequestStageInner {
    pub(super) fn new(osu: Arc<OsuInner>, req: Request) -> OsuResult<Self> {
        let ratelimiter = Arc::clone(&osu.ratelimiter);
        let generator = FutureRequestGenerator::new(osu, req)?;

        Ok(Self::Ratelimit(Ratelimit::new(ratelimiter, generator)))
    }
}

impl Future for OsuRequestStageInner {
    type Output = ControlFlow<Result<(Bytes, Arc<OsuInner>), OsuError>, OsuRequestStageInner>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            StageInnerProject::Ratelimit(ratelimit) => match ratelimit.poll(cx) {
                Poll::Ready(generator) => match generator.generate() {
                    Ok(req) => {
                        let future = generator.osu.http.request(req);
                        let in_flight = InFlight::new(future, generator);

                        Poll::Ready(ControlFlow::Continue(Self::InFlight(in_flight)))
                    }
                    Err(err) => Poll::Ready(ControlFlow::Break(Err(err))),
                },
                Poll::Pending => Poll::Pending,
            },
            StageInnerProject::InFlight(in_flight) => match in_flight.poll(cx) {
                Poll::Ready(InFlightOutput::Ratelimit(ratelimit)) => {
                    Poll::Ready(ControlFlow::Continue(Self::Ratelimit(ratelimit)))
                }
                Poll::Ready(InFlightOutput::Chunking(chunking)) => {
                    Poll::Ready(ControlFlow::Continue(Self::Chunking(chunking)))
                }
                Poll::Ready(InFlightOutput::Failed(err)) => {
                    Poll::Ready(ControlFlow::Break(Err(err)))
                }
                Poll::Pending => Poll::Pending,
            },
            StageInnerProject::Chunking(chunking) => chunking.poll(cx).map(ControlFlow::Break),
        }
    }
}

#[pin_project(project = StageProject, project_replace = StageReplace)]
pub(super) enum OsuFutureStage {
    User(#[pin] OsuRequestStageInner),
    Final(#[pin] OsuRequestStageInner),
    Failed(OsuError),
    Completed,
}

impl Future for OsuFutureStage {
    type Output = ControlFlow<OsuResult<(Bytes, Arc<OsuInner>)>, (UserExtended, Arc<OsuInner>)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.as_mut().project() {
                StageProject::User(mut stage) => match stage.as_mut().poll(cx) {
                    Poll::Ready(ControlFlow::Continue(next)) => {
                        stage.project_replace(next);
                    }
                    Poll::Ready(ControlFlow::Break(Ok((bytes, osu)))) => {
                        return match serde_json::from_slice(&bytes) {
                            Ok(user) => Poll::Ready(ControlFlow::Continue((user, osu))),
                            Err(source) => {
                                Poll::Ready(ControlFlow::Break(Err(OsuError::Parsing {
                                    bytes,
                                    source,
                                })))
                            }
                        }
                    }
                    Poll::Ready(ControlFlow::Break(Err(err))) => {
                        return Poll::Ready(ControlFlow::Break(Err(err)))
                    }
                    Poll::Pending => return Poll::Pending,
                },
                StageProject::Final(mut stage) => match stage.as_mut().poll(cx) {
                    Poll::Ready(ControlFlow::Continue(next)) => {
                        stage.project_replace(next);
                    }
                    Poll::Ready(ControlFlow::Break(res)) => {
                        return Poll::Ready(ControlFlow::Break(res))
                    }
                    Poll::Pending => return Poll::Pending,
                },
                StageProject::Failed(_) => {
                    let StageReplace::Failed(err) = self.project_replace(Self::Completed) else {
                        unreachable!()
                    };

                    return Poll::Ready(ControlFlow::Break(Err(err)));
                }
                StageProject::Completed => panic!("future already completed"),
            }
        }
    }
}
