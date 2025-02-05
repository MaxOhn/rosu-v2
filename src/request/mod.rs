macro_rules! poll_req {
    ($ty:ident => $ret:ty) => {
        impl ::std::future::Future for $ty<'_> {
            type Output = $crate::OsuResult<$ret>;

            fn poll(
                mut self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<Self::Output> {
                match self.fut {
                    Some(ref mut fut) => fut.as_mut().poll(cx),
                    None => {
                        let fut = self.start();

                        self.fut.get_or_insert(fut).as_mut().poll(cx)
                    }
                }
            }
        }
    };
}

mod beatmap;
mod comments;
mod event;
mod forum;
mod matches;
mod news;
mod ranking;
mod replay;
mod score;
mod seasonal_backgrounds;
mod serialize;
mod user;
mod wiki;

pub use beatmap::*;
use bytes::{Bytes, BytesMut};
pub use comments::*;
pub use event::*;
pub use forum::*;
pub use matches::*;
pub use news::*;
pub use ranking::*;
pub use replay::*;
pub use score::*;
pub use seasonal_backgrounds::*;
pub use user::*;
pub use wiki::*;

use crate::{routing::Route, OsuResult};

use itoa::{Buffer, Integer};
use serde::Serialize;
use std::{future::Future, pin::Pin};

type Pending<'a, T> = Pin<Box<dyn Future<Output = OsuResult<T>> + Send + Sync + 'a>>;

pub(crate) struct Request {
    pub query: Option<String>,
    pub route: Route,
    pub body: JsonBody,
    pub api_version: u32,
}

impl Request {
    #[allow(clippy::unreadable_literal)]
    const API_VERSION: u32 = 20220705;

    fn new(route: Route) -> Self {
        Self::with_body(route, JsonBody::new())
    }

    const fn with_body(route: Route, body: JsonBody) -> Self {
        Self {
            query: None,
            route,
            body,
            api_version: Self::API_VERSION,
        }
    }

    fn with_query(route: Route, query: String) -> Self {
        Self::with_query_and_body(route, query, JsonBody::new())
    }

    const fn with_query_and_body(route: Route, query: String, body: JsonBody) -> Self {
        Self {
            query: Some(query),
            route,
            body,
            api_version: Self::API_VERSION,
        }
    }

    fn api_version(&mut self, api_version: u32) {
        self.api_version = api_version;
    }
}

pub(crate) struct JsonBody {
    inner: BytesMut,
}

impl JsonBody {
    pub(crate) fn new() -> Self {
        Self {
            inner: BytesMut::new(),
        }
    }

    fn push_prefix(&mut self) {
        let prefix = if self.inner.is_empty() { b'{' } else { b',' };
        self.inner.extend_from_slice(&[prefix]);
    }

    fn push_key(&mut self, key: &[u8]) {
        self.push_prefix();
        self.inner.extend_from_slice(b"\"");
        self.inner.extend_from_slice(key);
        self.inner.extend_from_slice(b"\":");
    }

    fn push_value(&mut self, value: &[u8]) {
        self.inner.extend_from_slice(b"\"");
        self.inner.extend_from_slice(value);
        self.inner.extend_from_slice(b"\"");
    }

    pub(crate) fn push_str(&mut self, key: &str, value: &str) {
        self.inner.reserve(4 + key.len() + 2 + value.len());

        self.push_key(key.as_bytes());
        self.push_value(value.as_bytes());
    }

    pub(crate) fn push_int(&mut self, key: &str, int: impl Integer) {
        let mut buf = Buffer::new();
        let int = buf.format(int);

        self.inner.reserve(4 + key.len() + int.len());

        self.push_key(key.as_bytes());
        self.push_value(int.as_bytes());
    }

    pub(crate) fn into_bytes(mut self) -> Bytes {
        if !self.inner.is_empty() {
            self.inner.extend_from_slice(b"}");
        }

        self.inner.freeze()
    }
}

struct Query;

impl Query {
    fn encode<T: Serialize>(query: &T) -> String {
        serde_urlencoded::to_string(query).expect("serde_urlencoded should not fail")
    }
}
