use std::sync::Arc;

use bytes::Bytes;
use http_body_util::Full;
use hyper::{
    header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT},
    Request as HyperRequest,
};
use url::Url;

use crate::{
    client::OsuInner,
    error::OsuError,
    request::{Method, Request},
    OsuResult,
};

pub(super) struct FutureRequestGenerator {
    pub(super) osu: Arc<OsuInner>,
    method: Method,
    uri: Box<str>,
    token: HeaderValue,
    api_version: u32,
    body: Vec<u8>,
    pub(super) attempt: u8,
    #[cfg(feature = "metrics")]
    pub(super) route: &'static str,
}

pub(super) static MY_USER_AGENT: &str = concat!(
    "Rust API v2 (",
    env!("CARGO_PKG_REPOSITORY"),
    " v",
    env!("CARGO_PKG_VERSION"),
    ")",
);

pub(super) const APPLICATION_JSON: &str = "application/json";
pub(super) const X_API_VERSION: &str = "x-api-version";

impl FutureRequestGenerator {
    pub(super) fn new(osu: Arc<OsuInner>, req: Request) -> OsuResult<Self> {
        let Request {
            query,
            route,
            body,
            api_version,
        } = req;

        let (method, path) = route.as_parts();

        let mut url = format!("https://osu.ppy.sh/api/v2/{path}");

        if let Some(ref query) = query {
            url.push('?');
            url.push_str(query);
        }

        let url = Url::parse(&url).map_err(|source| OsuError::Url { source, url })?;
        debug!(%url, "Performing request...");

        let token_res = osu.token.get(|token| match token.access {
            Some(ref access) => match HeaderValue::from_str(access) {
                Ok(header) => Ok(header),
                Err(source) => Err(OsuError::CreatingTokenHeader { source }),
            },
            None => Err(OsuError::NoToken),
        });

        let token = token_res?;

        // `Url`'s parsing allocates a string based on the input length so we
        // are generally dealing with a `String` that has equal length and
        // capacity meaning we don't re-allocate when boxing the string.
        let uri = String::from(url).into_boxed_str();

        Ok(Self {
            osu,
            method,
            uri,
            token,
            api_version,
            body: body.into_bytes(),
            attempt: 0,
            #[cfg(feature = "metrics")]
            route: route.name(),
        })
    }

    pub(super) fn generate(&self) -> OsuResult<HyperRequest<Full<Bytes>>> {
        let len = self.body.len();

        let mut req = HyperRequest::builder()
            .method(self.method.into_hyper())
            .uri(self.uri.as_ref())
            .header(AUTHORIZATION, self.token.clone())
            .header(USER_AGENT, MY_USER_AGENT)
            .header(X_API_VERSION, self.api_version)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_LENGTH, len);

        if len > 0 {
            req = req.header(CONTENT_TYPE, APPLICATION_JSON);
        }

        let body = Full::new(Bytes::copy_from_slice(&self.body));

        req.body(body).map_err(OsuError::from)
    }
}
