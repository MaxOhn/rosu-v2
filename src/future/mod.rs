use std::{
    future::{Future, IntoFuture},
    ops::ControlFlow,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use pin_project::pin_project;

use crate::{
    request::{GetUser, Request, UserId},
    Osu, OsuResult,
};

use self::stage::{OsuFutureStage, OsuRequestStageInner};

pub(crate) use self::token::TokenFuture;

pub use self::traits::*;

mod request_generator;
mod stage;
mod token;
mod traits;

type FromUserFn<T> = fn(u32, <T as OsuFutureData>::FromUserData) -> Request;

struct FromUser<T: OsuFutureData> {
    data: T::FromUserData,
    f: FromUserFn<T>,
}

type PostProcessFn<T> = fn(
    <T as OsuFutureData>::FromBytes,
    <T as OsuFutureData>::PostProcessData,
) -> OsuResult<<T as OsuFutureData>::OsuOutput>;

struct PostProcess<T: OsuFutureData> {
    data: T::PostProcessData,
    f: PostProcessFn<T>,
}

/// Awaitable [`Future`] to fetch and process data from an endpoint.
///
/// When fetching from user endpoints by name instead of id, the [`OsuFuture`]
/// might first perform a request to fetch the user itself, and then perform
/// the actual request by using the fetched user id. If the `cache` feature
/// is enabled, fetched user data will be stored to potentially prevent
/// intermediate user requests later on.
#[pin_project]
pub struct OsuFuture<T: OsuFutureData> {
    #[pin]
    stage: OsuFutureStage,
    from_user: Option<FromUser<T>>,
    post_process: Option<PostProcess<T>>,
}

impl<T: OsuFutureData> OsuFuture<T> {
    /// Creates a new [`OsuFuture`] from the given [`Request`].
    pub(crate) fn new(
        osu: &Osu,
        req: Request,
        post_process_data: T::PostProcessData,
        post_process_fn: PostProcessFn<T>,
    ) -> Self {
        let osu = Arc::clone(&osu.inner);

        Self {
            stage: OsuRequestStageInner::new(osu, req)
                .map_or_else(OsuFutureStage::Failed, OsuFutureStage::Final),
            from_user: None,
            post_process: Some(PostProcess {
                data: post_process_data,
                f: post_process_fn,
            }),
        }
    }

    /// Creates a new [`OsuFuture`] which might fetch a user first if the
    /// given [`UserId`] is a name that has not been cached yet.
    pub(crate) fn from_user_id(
        osu: &Osu,
        user_id: UserId,
        from_user_data: T::FromUserData,
        from_user_fn: FromUserFn<T>,
        post_process_data: T::PostProcessData,
        post_process_fn: PostProcessFn<T>,
    ) -> Self {
        #[cfg(not(feature = "cache"))]
        let get_user_id: fn(UserId) -> UserId = std::convert::identity;

        #[cfg(feature = "cache")]
        fn get_user_id(mut user_id: UserId, osu: &Osu) -> UserId {
            if let UserId::Name(ref mut name) = user_id {
                name.make_ascii_lowercase();

                if let Some(id) = osu.inner.cache.get(name) {
                    return UserId::Id(*id);
                }
            }

            user_id
        }

        match get_user_id(
            user_id,
            #[cfg(feature = "cache")]
            osu,
        ) {
            UserId::Id(user_id) => {
                let req = from_user_fn(user_id, from_user_data);

                Self::new(osu, req, post_process_data, post_process_fn)
            }
            user_id @ UserId::Name(_) => {
                #[cfg(not(feature = "cache"))]
                {
                    static NOTIF: std::sync::Once = std::sync::Once::new();

                    // In case users intend to fetch frequently from user
                    // endpoints by username but weren't aware either that
                    // they have the `cache` feature disabled or that
                    // disabling the feature will add an additional request
                    // on every fetch, let's remind them a single time.
                    NOTIF.call_once(|| {
                        warn!(
                            "Fetching from a user endpoint by username will \
                            always perform two requests because the `cache` \
                            feature is not enabled"
                        );
                    });
                }

                let osu = Arc::clone(&osu.inner);
                let req = GetUser::create_request(user_id, None);

                Self {
                    stage: OsuRequestStageInner::new(osu, req)
                        .map_or_else(OsuFutureStage::Failed, OsuFutureStage::User),
                    from_user: Some(FromUser {
                        data: from_user_data,
                        f: from_user_fn,
                    }),
                    post_process: Some(PostProcess {
                        data: post_process_data,
                        f: post_process_fn,
                    }),
                }
            }
        }
    }
}

impl<T: OsuFutureData> Future for OsuFuture<T> {
    type Output = <T as IntoFuture>::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();

        match this.stage.as_mut().poll(cx) {
            Poll::Ready(ControlFlow::Break(Ok((bytes, osu)))) => {
                let res = <T::FromBytes>::from_bytes(bytes)?;
                let PostProcess { data, f } =
                    this.post_process.take().expect("missing post_process");

                let value = f(res, data)?;

                #[cfg(feature = "cache")]
                crate::model::ContainedUsers::apply_to_users(&value, |id, name| {
                    osu.update_cache(id, name);
                });

                // Preventing "unused variable" lint w/o `cache` feature
                let _ = osu;

                Poll::Ready(Ok(value))
            }
            Poll::Ready(ControlFlow::Continue((user, osu))) => {
                #[cfg(feature = "cache")]
                osu.update_cache(user.user_id, &user.username);

                #[cfg(feature = "metrics")]
                // Technically, using a gauge and setting it to
                // `osu.cache.len()` would be more correct but since
                // `DashMap::len` is a non-trivial call, it should be fine
                // to increment a counter. This works because we're only in
                // this path if the cache did not contain the username in
                // the first place, meaning we indeed add a new entry.
                ::metrics::counter!(crate::metrics::USERNAME_CACHE_SIZE).increment(1);

                let FromUser { data, f } = this.from_user.take().expect("missing from_user");
                let req = f(user.user_id, data);

                let next = OsuRequestStageInner::new(osu, req)?;
                this.stage.project_replace(OsuFutureStage::Final(next));

                self.poll(cx)
            }
            Poll::Ready(ControlFlow::Break(Err(err))) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) const fn noop_post_process<T>(value: T, _: ()) -> OsuResult<T> {
    Ok(value)
}
