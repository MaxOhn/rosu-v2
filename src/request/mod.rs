macro_rules! poll_req {
    ($ty: ty, $ret: ty) => {
        impl ::std::future::Future for $ty {
            type Output = $crate::OsuResult<$ret>;

            fn poll(
                mut self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<Self::Output> {
                loop {
                    if let Some(fut) = self.as_mut().fut.as_mut() {
                        return fut.as_mut().poll(cx);
                    } else if let Err(why) = self.as_mut().start() {
                        return ::std::task::Poll::Ready(Err(why));
                    }
                }
            }
        }
    };
}

mod beatmap;
mod comments;
mod multiplayer;
mod ranking;
mod user;
mod wiki;

pub use beatmap::{GetBeatmap, GetBeatmapScores, GetBeatmapUserScore};
pub use comments::GetComments;
pub use multiplayer::{GetScore, GetScores, GetUserHighScore};
pub use ranking::{GetRankings, GetSpotlights};
pub use user::{
    GetRecentEvents, GetUser, GetUserBeatmapsets, GetUserKudosu, GetUserMostPlayed, GetUserScores,
    GetUsers, UserId,
};
pub use wiki::GetWikiPage;

use crate::{routing::Route, OsuResult};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::{borrow::Cow, future::Future, iter::Extend, pin::Pin, vec::IntoIter};

type Pending<'a, T> = Pin<Box<dyn Future<Output = OsuResult<T>> + Send + 'a>>;

#[derive(Debug, Default)]
pub(crate) struct Query(Vec<(&'static str, Cow<'static, str>)>);

impl Query {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn push(&mut self, key: &'static str, value: impl Into<Cow<'static, str>>) {
        self.0.push((key, value.into()));
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl IntoIterator for Query {
    type Item = (&'static str, Cow<'static, str>);
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Into<Cow<'static, str>>> Extend<(&'static str, T)> for Query {
    #[inline]
    fn extend<I: IntoIterator<Item = (&'static str, T)>>(&mut self, iter: I) {
        self.0
            .extend(iter.into_iter().map(|(key, val)| (key, val.into())));
    }
}

impl Serialize for Query {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(self.0.len()))?;

        for pair in &self.0 {
            seq.serialize_element(pair)?;
        }

        seq.end()
    }
}

#[derive(Debug)]
pub(crate) struct Request {
    pub body: Option<Vec<u8>>,
    pub query: Query,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub method: Method,
    pub path: Cow<'static, str>,
}

impl From<Route> for Request {
    fn from(route: Route) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: None,
            query: Query::new(),
            headers: None,
            method,
            path,
        }
    }
}

impl From<(Vec<u8>, Route)> for Request {
    fn from((body, route): (Vec<u8>, Route)) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: Some(body),
            query: Query::new(),
            headers: None,
            method,
            path,
        }
    }
}

impl From<(Query, Route)> for Request {
    fn from((query, route): (Query, Route)) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: None,
            query,
            headers: None,
            method,
            path,
        }
    }
}

impl From<(Vec<u8>, Query, Route)> for Request {
    fn from((body, query, route): (Vec<u8>, Query, Route)) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: Some(body),
            query,
            headers: None,
            method,
            path,
        }
    }
}

impl From<(HeaderMap<HeaderValue>, Route)> for Request {
    fn from((headers, route): (HeaderMap<HeaderValue>, Route)) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: None,
            query: Query::new(),
            headers: Some(headers),
            method,
            path,
        }
    }
}

impl From<(Vec<u8>, HeaderMap<HeaderValue>, Route)> for Request {
    fn from((body, headers, route): (Vec<u8>, HeaderMap<HeaderValue>, Route)) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: Some(body),
            query: Query::new(),
            headers: Some(headers),
            method,
            path,
        }
    }
}
