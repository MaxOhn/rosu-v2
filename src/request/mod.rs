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
use serde::Serialize;
use std::{borrow::Cow, future::Future, pin::Pin};

type Pending<'a, T> = Pin<Box<dyn Future<Output = OsuResult<T>> + Send + 'a>>;

#[derive(Debug)]
pub(crate) struct Request {
    pub query: Option<Query>,
    pub method: Method,
    pub path: Cow<'static, str>,
}

impl Request {
    #[inline]
    fn new(route: Route) -> Self {
        let (method, path) = route.into_parts();

        Self {
            query: None,
            method,
            path,
        }
    }

    #[inline]
    fn query(mut self, query: Query) -> Self {
        self.query.replace(query);

        self
    }
}

#[derive(Debug)]
pub(crate) struct Query {
    query: Vec<u8>,
}

impl Default for Query {
    #[inline]
    fn default() -> Self {
        Self { query: vec![b'{'] }
    }
}

impl Query {
    #[inline]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub(crate) fn finish(mut self) -> Vec<u8> {
        if self.query.len() > 1 {
            self.query.pop();
        }

        self.query.push(b'}');

        self.query
    }

    #[inline]
    pub(crate) fn push(&mut self, key: &str, value: &impl Serialize) -> &mut Self {
        self.query.reserve(5 + key.len());

        self.query.push(b'"');
        self.query.extend_from_slice(key.as_bytes());
        self.query.push(b'"');
        self.query.push(b':');
        let _ = serde_json::to_writer(&mut self.query, value);
        self.query.push(b',');

        self
    }
}
