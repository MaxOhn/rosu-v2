use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[cfg(feature = "metrics")]
use std::time::Instant;

use bytes::{BufMut, BytesMut};
use http_body_util::Full;
use hyper::{
    body::Bytes,
    header::{ACCEPT, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT},
    Request as HyperRequest, StatusCode,
};
use hyper_util::client::legacy::ResponseFuture as HyperResponseFuture;
use pin_project::pin_project;
use serde::Serialize;
use tokio::time::Timeout;

use crate::{
    client::{Authorization, OsuInner, Scopes, TokenResponse},
    error::OsuError,
    OsuResult,
};

use super::{
    request_generator::{APPLICATION_JSON, MY_USER_AGENT},
    stage::Chunking,
};

struct TokenRequestGenerator<'a> {
    body: Bytes,
    base_url: &'a str,
}

impl<'a> TokenRequestGenerator<'a> {
    const fn new(body: Bytes, base_url: &'a str) -> Self {
        Self { body, base_url }
    }

    fn generate(self) -> OsuResult<HyperRequest<Full<Bytes>>> {
        let len = self.body.len();
        let body = Full::new(self.body);
        let url = format!("{}/oauth/token", self.base_url);

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
        #[derive(Serialize)]
        struct CodeGrant<'a> {
            client_id: u64,
            client_secret: &'a str,
            scope: &'a str,
            grant_type: &'static str,
        }

        let mut scopes = String::new();
        Scopes::Public.format(&mut scopes, ' ');

        let (client_id, client_secret) = credentials(&osu)?;

        let body = CodeGrant {
            client_id,
            client_secret,
            scope: &scopes,
            grant_type: "client_credentials",
        };

        let body = serialize_body(&body)?;

        Ok(Self::new(osu, body))
    }

    pub(crate) fn new_user(osu: Arc<OsuInner>, auth: &Authorization) -> Result<Self, OsuError> {
        #[derive(Serialize)]
        struct CodeGrant<'a> {
            client_id: u64,
            client_secret: &'a str,
            scope: &'a str,
            grant_type: &'static str,
            code: &'a str,
            redirect_uri: &'a str,
        }

        let mut scopes = String::new();
        auth.scopes.format(&mut scopes, ' ');

        let (client_id, client_secret) = credentials(&osu)?;

        let body = CodeGrant {
            client_id,
            client_secret,
            scope: &scopes,
            grant_type: "authorization_code",
            code: &auth.code,
            redirect_uri: &auth.redirect_uri,
        };

        let body = serialize_body(&body)?;

        Ok(Self::new(osu, body))
    }

    pub(crate) fn new_refresh(osu: Arc<OsuInner>, refresh: &str) -> Result<Self, OsuError> {
        #[derive(Serialize)]
        struct CodeGrant<'a> {
            client_id: u64,
            client_secret: &'a str,
            grant_type: &'static str,
            refresh_token: &'a str,
        }

        let (client_id, client_secret) = credentials(&osu)?;

        let body = CodeGrant {
            client_id,
            client_secret,
            grant_type: "refresh_token",
            refresh_token: refresh,
        };

        let body = serialize_body(&body)?;

        Ok(Self::new(osu, body))
    }

    fn new(osu: Arc<OsuInner>, body: Bytes) -> Self {
        let inner = match TokenRequestGenerator::new(body, &osu.base_url).generate() {
            Ok(req) => TokenFutureInner::InFlight(TokenInFlight::new(osu.http.request(req), osu)),
            Err(err) => TokenFutureInner::Completed(Some(err)),
        };

        Self { inner }
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

fn credentials(osu: &OsuInner) -> OsuResult<(u64, &str)> {
    let client_id = osu.client_id.ok_or(OsuError::BuilderMissingId)?;

    let client_secret = osu
        .client_secret
        .as_deref()
        .ok_or(OsuError::BuilderMissingSecret)?;

    Ok((client_id, client_secret))
}

fn serialize_body<T: Serialize>(body: &T) -> OsuResult<Bytes> {
    let mut bytes = BytesMut::new();

    if let Err(err) = serde_json::to_writer((&mut bytes).writer(), body) {
        return Err(OsuError::Serialize(err));
    }

    Ok(bytes.freeze())
}
