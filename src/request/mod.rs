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
mod multiplayer;
mod news;
mod ranking;
mod seasonal_backgrounds;
mod user;
mod wiki;

pub use beatmap::*;
pub use comments::*;
pub use forum::*;
pub use matches::*;
pub use multiplayer::*;
pub use news::*;
pub use ranking::*;
pub use seasonal_backgrounds::*;
pub use user::*;
pub use wiki::*;

use crate::{routing::Route, OsuResult};

use hyper::Method;
use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result, Write},
    future::Future,
    pin::Pin,
};

type Pending<'a, T> = Pin<Box<dyn Future<Output = OsuResult<T>> + Send + 'a>>;

#[derive(Debug)]
pub(crate) struct Request {
    pub query: Query,
    pub method: Method,
    pub path: Cow<'static, str>,
}

impl Request {
    #[inline]
    fn new(route: Route) -> Self {
        Self::with_query(route, Query::default())
    }

    #[inline]
    fn with_query(route: Route, query: Query) -> Self {
        let (method, path) = route.into_parts();

        Self {
            query,
            method,
            path,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Query {
    query: String,
}

impl Default for Query {
    #[inline]
    fn default() -> Self {
        Self {
            query: String::new(),
        }
    }
}

impl Query {
    #[inline]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub(crate) fn push(&mut self, key: &str, value: impl Display) {
        self.query.reserve(2 + key.len());

        self.query.push_str(key);
        self.query.push('=');
        let _ = write!(self.query, "{}", value);

        self.query.push('&');
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.query.is_empty() {
            return Ok(());
        }

        write!(f, "?{}", &self.query[..self.query.len() - 1])
    }
}
