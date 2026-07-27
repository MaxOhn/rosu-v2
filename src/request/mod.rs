/// Implements [`OsuFutureData`] and [`IntoFuture`] for requests.
///
/// [`OsuFutureData`]: crate::future::OsuFutureData
/// [`IntoFuture`]: std::future::IntoFuture
macro_rules! into_future {
    // New OsuFuture with optional post processing
    (
        |$self:ident: $ty:ty| -> $from_bytes:ty { $( $req_body:tt )+ }
        $( =>
            |
                $from_bytes_arg:tt,
                $post_process_data_arg:tt $(: $post_process_data:ty )?
            | -> $output:ty { $( $post_process_body:tt )+ }
        )?
    ) => {
        impl crate::future::OsuFutureData for $ty {
            type FromBytes = $from_bytes;
            type OsuOutput = into_future!(OUTPUT_TY $from_bytes $( | $output )?);
            type FromUserData = ();
            type PostProcessData = into_future!(POST_PROCESS_DATA $( $( $post_process_data )? )?);
        }

        impl std::future::IntoFuture for $ty {
            type Output = crate::OsuResult<into_future!(OUTPUT_TY $from_bytes $( | $output )?)>;
            type IntoFuture = crate::future::OsuFuture<Self>;

            fn into_future($self) -> Self::IntoFuture {
                let res = { $( $req_body )* };
                let (req, data) =
                    crate::future::IntoPostProcessData::into_data(res);

                let post_process_fn = into_future!(POST_PROCESS_FN $(
                    |
                        $from_bytes_arg: $from_bytes,
                        $post_process_data_arg $(: $post_process_data )?
                    | { $( $post_process_body )* }
                )?);

                crate::future::OsuFuture::new(
                    $self.osu,
                    req,
                    data,
                    post_process_fn,
                )
            }
        }
    };

    // OsuFuture from UserId with optional post processing
    (
        |$self:ident: $ty:ty| -> $from_bytes:ty {
            $from_user_data:ident {
                $( $data_field_name:ident: $data_field_ty:ty = $data_field_value:expr, )*
            }
        } => |$user_id_arg:tt, $from_user_data_arg:tt| { $( $req_body:tt )* }
        $(
            => |$from_bytes_arg:tt, $post_process_data_arg:tt $(: $post_process_data:ty )?|
                -> $output:ty { $( $post_process_body:tt )* }
        )?
    ) => {
        #[doc(hidden)]
        pub struct $from_user_data {
            $( $data_field_name: $data_field_ty, )*
        }

        impl crate::future::OsuFutureData for $ty {
            type FromBytes = $from_bytes;
            type OsuOutput = into_future!(OUTPUT_TY $from_bytes $( | $output )?);
            type FromUserData = $from_user_data;
            type PostProcessData = into_future!(POST_PROCESS_DATA $( $( $post_process_data )? )?);
        }

        impl std::future::IntoFuture for $ty {
            type Output = crate::OsuResult<into_future!(OUTPUT_TY $from_bytes $( | $output )?)>;
            type IntoFuture = crate::future::OsuFuture<Self>;

            fn into_future($self) -> Self::IntoFuture {
                let from_user_data = $from_user_data {
                    $( $data_field_name: $data_field_value, )*
                };

                let from_user_fn = |$user_id_arg: u32, $from_user_data_arg: $from_user_data| {
                    $( $req_body )*
                };

                let post_process_fn = into_future!(POST_PROCESS_FN $(
                    |
                        $from_bytes_arg: $from_bytes,
                        $post_process_data_arg $(: $post_process_data )?
                    | { $( $post_process_body )* }
                )?);

                crate::future::OsuFuture::from_user_id(
                    $self.osu,
                    $self.user_id,
                    from_user_data,
                    from_user_fn,
                    (), // FIXME: handle different post process data types
                    post_process_fn,
                )
            }
        }
    };

    // Helper rules

    ( OUTPUT_TY $output:ty ) => {
        $output
    };
    ( OUTPUT_TY $from_bytes:ty | $output:ty ) => {
        $output
    };
    ( POST_PROCESS_DATA ) => {
        ()
    };
    ( POST_PROCESS_DATA $data:ty ) => {
        $data
    };
    ( POST_PROCESS_FN ) => {
        crate::future::noop_post_process
    };
    ( POST_PROCESS_FN
        |$from_bytes_arg:tt: $from_bytes:ty, $data_arg:tt $(: $data:ty )?|
            { $( $post_process_body:tt )* }
    ) => {
        |
            #[allow(unused_mut)]
            mut $from_bytes_arg: $from_bytes,
            $data_arg: into_future!(POST_PROCESS_DATA $( $data )?),
        | { $( $post_process_body )* }
    };
}

use bytes::Bytes;
use serde::Serialize;

use crate::routing::Route;

pub use crate::future::OsuFuture;

pub use self::{
    beatmap::*, changelog::*, chat::*, comments::*, event::*, forum::*, matches::*, multiplayer::*,
    news::*, ranking::*, replay::*, score::*, seasonal_backgrounds::*, user::*, wiki::*,
};

mod beatmap;
mod changelog;
mod chat;
mod comments;
mod event;
mod forum;
mod matches;
mod multiplayer;
mod news;
mod ranking;
mod replay;
mod score;
mod seasonal_backgrounds;
mod serialize;
mod user;
mod wiki;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Method {
    Get,
    Delete,
    Post,
    Put,
}

impl Method {
    pub const fn into_hyper(self) -> hyper::Method {
        match self {
            Method::Get => hyper::Method::GET,
            Method::Delete => hyper::Method::DELETE,
            Method::Post => hyper::Method::POST,
            Method::Put => hyper::Method::PUT,
        }
    }
}

pub(crate) struct Request {
    pub query: Option<String>,
    pub route: Route,
    pub body: Bytes,
    pub api_version: u32,
}

impl Request {
    #[allow(clippy::unreadable_literal)]
    const API_VERSION: u32 = 20220705;

    const fn new(route: Route) -> Self {
        Self::with_body(route, Bytes::new())
    }

    const fn with_body(route: Route, body: Bytes) -> Self {
        Self {
            query: None,
            route,
            body,
            api_version: Self::API_VERSION,
        }
    }

    const fn with_query(route: Route, query: String) -> Self {
        Self::with_query_and_body(route, query, Bytes::new())
    }

    const fn with_query_and_body(route: Route, query: String, body: Bytes) -> Self {
        Self {
            query: Some(query),
            route,
            body,
            api_version: Self::API_VERSION,
        }
    }

    const fn api_version(&mut self, api_version: u32) {
        self.api_version = api_version;
    }
}

struct Query;

impl Query {
    fn encode<T: Serialize>(query: &T) -> String {
        serde_urlencoded::to_string(query).expect("serde_urlencoded should not fail")
    }
}
