use std::future::IntoFuture;

use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::{error::OsuError, model::ContainedUsers, request::Request, OsuResult};

/// Converting `Self` into an [`OsuFuture<Self>`].
///
/// [`OsuFuture<Self>`]: crate::future::OsuFuture
#[allow(
    private_bounds,
    reason = "users should not implement this trait anyway"
)]
pub trait OsuFutureData: IntoFuture<Output = OsuResult<Self::OsuOutput>> {
    /// The received [`Bytes`] will be turned into this type.
    type FromBytes: FromBytes;
    /// Post processing will convert [`Self::FromBytes`] into this type.
    type OsuOutput: ContainedUsers;
    /// Auxiliary data to create a request from a user.
    type FromUserData;
    /// Auxiliary data used for post processing.
    type PostProcessData;
}

/// Converting [`Bytes`] into `OsuResult<Self>`.
pub(crate) trait FromBytes: Sized {
    fn from_bytes(bytes: Bytes) -> OsuResult<Self>;
}

/// [`Bytes`] wrapper to implement [`FromBytes`] for bytes.
#[doc(hidden)]
pub struct BytesWrap(pub(crate) Bytes);

impl FromBytes for BytesWrap {
    fn from_bytes(bytes: Bytes) -> OsuResult<Self> {
        Ok(Self(bytes))
    }
}

impl<T: DeserializeOwned> FromBytes for T {
    fn from_bytes(bytes: Bytes) -> OsuResult<Self> {
        serde_json::from_slice(&bytes).map_err(|source| OsuError::Parsing { bytes, source })
    }
}

/// Auxiliary trait to simplify logic within the [`into_future!`] macro.
pub(crate) trait IntoPostProcessData<D> {
    fn into_data(self) -> (Request, D);
}

impl IntoPostProcessData<()> for Request {
    fn into_data(self) -> (Request, ()) {
        (self, ())
    }
}

impl<D> IntoPostProcessData<D> for (Request, D) {
    fn into_data(self) -> (Request, D) {
        self
    }
}
