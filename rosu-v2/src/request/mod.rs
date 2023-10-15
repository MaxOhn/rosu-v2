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
mod forum;
mod matches;
mod news;
mod ranking;
mod replay;
mod seasonal_backgrounds;
mod serialize;
mod user;
mod wiki;

pub use beatmap::*;
pub use comments::*;
pub use forum::*;
pub use matches::*;
pub use news::*;
pub use ranking::*;
pub use replay::*;
pub use seasonal_backgrounds::*;
pub use user::*;
pub use wiki::*;

use crate::{routing::Route, OsuResult};

use serde::Serialize;
use std::{
    fmt::{Display, Write},
    future::Future,
    pin::Pin,
};

type Pending<'a, T> = Pin<Box<dyn Future<Output = OsuResult<T>> + Send + Sync + 'a>>;

pub(crate) struct Request {
    pub query: Option<String>,
    pub route: Route,
    pub body: Body,
}

impl Request {
    fn new(route: Route) -> Self {
        Self::with_body(route, Body::new())
    }

    fn with_body(route: Route, body: Body) -> Self {
        Self {
            query: None,
            route,
            body,
        }
    }

    fn with_query(route: Route, query: String) -> Self {
        Self::with_query_and_body(route, query, Body::new())
    }

    fn with_query_and_body(route: Route, query: String, body: Body) -> Self {
        Self {
            query: Some(query),
            route,
            body,
        }
    }
}

pub(crate) struct Body {
    inner: String,
}

impl Body {
    pub(crate) fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    fn push_prefix(&mut self) {
        if self.inner.is_empty() {
            self.inner.push('{');
        } else {
            self.inner.push(',');
        }
    }

    fn push_key(&mut self, key: &str) {
        self.push_prefix();
        self.inner.push('"');
        self.inner.push_str(key);
        self.inner.push_str("\":");
    }

    pub(crate) fn push_with_quotes(&mut self, key: &str, value: impl Display) {
        self.push_key(key);
        let _ = write!(self.inner, r#""{value}""#);
    }

    pub(crate) fn push_without_quotes(&mut self, key: &str, value: impl Display) {
        self.push_key(key);
        let _ = write!(self.inner, "{value}");
    }

    pub(crate) fn into_bytes(mut self) -> Vec<u8> {
        if !self.inner.is_empty() {
            self.inner.push('}');
        }

        self.inner.into_bytes()
    }
}

struct Query;

impl Query {
    fn encode<T: Serialize>(query: &T) -> String {
        serde_urlencoded::to_string(query).expect("serde_urlencoded should not fail")
    }
}
