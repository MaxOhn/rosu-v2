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

use std::{
    fmt::{Display, Formatter, Result, Write},
    future::Future,
    pin::Pin,
};

type Pending<'a, T> = Pin<Box<dyn Future<Output = OsuResult<T>> + Send + Sync + 'a>>;

pub(crate) struct Request {
    pub query: Query,
    pub route: Route,
    pub body: Body,
}

impl Request {
    fn new(route: Route) -> Self {
        Self::with_query_and_body(route, Query::default(), Body::default())
    }

    fn with_query(route: Route, query: Query) -> Self {
        Self::with_query_and_body(route, query, Body::default())
    }

    fn with_body(route: Route, body: Body) -> Self {
        Self::with_query_and_body(route, Query::default(), body)
    }

    fn with_query_and_body(route: Route, query: Query, body: Body) -> Self {
        Self { query, route, body }
    }
}

#[derive(Default)]
pub(crate) struct Body {
    inner: String,
}

impl Body {
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

#[derive(Default)]
pub(crate) struct Query {
    query: String,
}

impl Query {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, key: &str, value: impl Display) {
        self.query.push_str(key);
        self.query.push('=');
        let _ = write!(self.query, "{}", value);
        self.query.push('&');
    }
}

impl Display for Query {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.query.is_empty() {
            return Ok(());
        }

        f.write_char('?')?;
        f.write_str(&self.query[..self.query.len() - 1])
    }
}
