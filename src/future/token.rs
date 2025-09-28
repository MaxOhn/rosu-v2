use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[cfg(feature = "metrics")]
use std::time::Instant;

use http_body_util::Full;
use hyper::{
    body::Bytes,
    header::{ACCEPT, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT},
    Request as HyperRequest, StatusCode,
};
use hyper_util::client::legacy::ResponseFuture as HyperResponseFuture;
use pin_project::pin_project;
use tokio::time::Timeout;

use crate::{
    client::{Authorization, OsuInner, Scopes, TokenResponse},
    error::OsuError,
    request::JsonBody,
    OsuResult,
};

use super::{
    request_generator::{APPLICATION_JSON, MY_USER_AGENT},
    stage::Chunking,
};

struct TokenRequestGenerator {
    body: Vec<u8>,
}

impl TokenRequestGenerator {
    fn new(osu: &OsuInner, mut body: JsonBody) -> Result<Self, OsuError> {
        match osu.client_id {
            Some(client_id) => body.push_int("client_id", client_id),
            None => return Err(OsuError::BuilderMissingId),
        }

        match &osu.client_secret {
            Some(client_secret) => body.push_str("client_secret", client_secret),
            None => return Err(OsuError::BuilderMissingSecret),
        }

        Ok(Self {
            body: body.into_bytes(),
        })
    }

    fn generate(self) -> OsuResult<HyperRequest<Full<Bytes>>> {
        let len = self.body.len();
        let body = Full::new(Bytes::from(self.body));
        let url = "https://osu.ppy.sh/oauth/token";

        HyperRequest::post(url)
            .header(USER_AGENT, MY_USER_AGENT)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(CONTENT_LENGTH, len)
            .body(body)
            .map_err(OsuError::from)
    }
}

#[pin_project]
struct TokenInFlight {
    #[pin]
    future: Timeout<HyperResponseFuture>,
    osu: Option<Arc<OsuInner>>,
    #[cfg(feature = "metrics")]
    start: Option<Instant>,
}

impl TokenInFlight {
    fn new(future: HyperResponseFuture, osu: Arc<OsuInner>) -> Self {
        Self {
            future: tokio::time::timeout(osu.timeout, future),
            osu: Some(osu),
            #[cfg(feature = "metrics")]
            start: None,
        }
    }
}

impl Future for TokenInFlight {
    type Output = OsuResult<Chunking>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        #[cfg(feature = "metrics")]
        let start = *this.start.get_or_insert_with(Instant::now);

        match this.future.poll(cx) {
            Poll::Ready(Ok(Ok(resp))) => {
                match resp.status() {
                    StatusCode::SERVICE_UNAVAILABLE => {
                        return Poll::Ready(Err(OsuError::ServiceUnavailable {
                            body: resp.into_body(),
                        }))
                    }
                    StatusCode::TOO_MANY_REQUESTS => warn!("429 response: {resp:?}"),
                    _ => {}
                }

                let osu = this.osu.take().expect("missing osu");

                Poll::Ready(Ok(Chunking::new(
                    resp,
                    osu,
                    #[cfg(feature = "metrics")]
                    super::stage::ChunkingMetrics {
                        start,
                        route: "PostToken",
                    },
                )))
            }
            Poll::Ready(Ok(Err(source))) => Poll::Ready(Err(OsuError::Request { source })),
            Poll::Ready(Err(_)) => Poll::Ready(Err(OsuError::RequestTimeout)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[pin_project(project = TokenProject, project_replace = TokenReplace)]
enum TokenFutureInner {
    InFlight(#[pin] TokenInFlight),
    Chunking(#[pin] Chunking),
    Completed(Option<OsuError>),
}

#[pin_project]
pub(crate) struct TokenFuture {
    #[pin]
    inner: TokenFutureInner,
}

impl TokenFuture {
    pub(crate) fn new_client(osu: Arc<OsuInner>) -> Result<Self, OsuError> {
        let mut body = JsonBody::new();

        body.push_str("grant_type", "client_credentials");
        let mut scopes = String::new();
        Scopes::Public.format(&mut scopes, ' ');
        body.push_str("scope", &scopes);

        Self::new(osu, body)
    }

    pub(crate) fn new_user(osu: Arc<OsuInner>, auth: &Authorization) -> Result<Self, OsuError> {
        let mut body = JsonBody::new();

        body.push_str("grant_type", "authorization_code");
        body.push_str("redirect_uri", &auth.redirect_uri);
        body.push_str("code", &auth.code);
        let mut scopes = String::new();
        auth.scopes.format(&mut scopes, ' ');
        body.push_str("scope", &scopes);

        Self::new(osu, body)
    }

    pub(crate) fn new_refresh(osu: Arc<OsuInner>, refresh: &str) -> Result<Self, OsuError> {
        let mut body = JsonBody::new();

        body.push_str("grant_type", "refresh_token");
        body.push_str("refresh_token", refresh);

        Self::new(osu, body)
    }

    fn new(osu: Arc<OsuInner>, body: JsonBody) -> Result<Self, OsuError> {
        let inner = match TokenRequestGenerator::new(&osu, body)?.generate() {
            Ok(req) => TokenFutureInner::InFlight(TokenInFlight::new(osu.http.request(req), osu)),
            Err(err) => TokenFutureInner::Completed(Some(err)),
        };

        Ok(Self { inner })
    }
}

impl Future for TokenFuture {
    type Output = OsuResult<TokenResponse>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();

        match this.inner.as_mut().project() {
            TokenProject::InFlight(in_flight) => match in_flight.poll(cx) {
                Poll::Ready(Ok(chunking)) => {
                    this.inner
                        .project_replace(TokenFutureInner::Chunking(chunking));

                    self.poll(cx)
                }
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending,
            },
            TokenProject::Chunking(chunking) => match chunking.poll(cx) {
                Poll::Ready(Ok((bytes, _))) => {
                    let res = serde_json::from_slice(&bytes)
                        .map_err(|source| OsuError::Parsing { bytes, source });

                    Poll::Ready(res)
                }
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending,
            },
            TokenProject::Completed(err) => match err.take() {
                Some(source) => Poll::Ready(Err(source)),
                None => panic!("future already completed"),
            },
        }
    }
}
