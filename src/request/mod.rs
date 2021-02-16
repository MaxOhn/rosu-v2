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

pub mod beatmap;
pub mod comments;
pub mod multiplayer;
pub mod ranking;
pub mod user;
mod wiki;

pub use beatmap::{GetBeatmap, GetBeatmapScores, GetBeatmapUserScore};
pub use comments::GetComments;
pub use multiplayer::{GetScore, GetScores, GetUserHighScore};
pub use ranking::{GetRankings, GetSpotlights};
pub use user::{
    GetRecentEvents, GetUser, GetUserBeatmapsets, GetUserKudosu, GetUserScores, GetUsers, UserId,
};
pub use wiki::GetWikiPage;

use crate::{routing::Route, OsuResult};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart::Form,
    Method,
};
use std::{borrow::Cow, future::Future, pin::Pin};

type Pending<'a, T> = Pin<Box<dyn Future<Output = OsuResult<T>> + Send + 'a>>;

#[derive(Debug)]
pub(crate) struct Request {
    pub body: Option<Vec<u8>>,
    pub form: Option<Form>,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub method: Method,
    pub path: Cow<'static, str>,
}

impl From<Route> for Request {
    fn from(route: Route) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: None,
            form: None,
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
            form: None,
            headers: None,
            method,
            path,
        }
    }
}

impl From<(Form, Route)> for Request {
    fn from((form, route): (Form, Route)) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: None,
            form: Some(form),
            headers: None,
            method,
            path,
        }
    }
}

impl From<(Vec<u8>, Form, Route)> for Request {
    fn from((body, form, route): (Vec<u8>, Form, Route)) -> Self {
        let (method, path) = route.into_parts();

        Self {
            body: Some(body),
            form: Some(form),
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
            form: None,
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
            form: None,
            headers: Some(headers),
            method,
            path,
        }
    }
}
