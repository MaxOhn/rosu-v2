#![cfg(feature = "metrics")]

use metrics::{describe_histogram, Unit};

pub(crate) const RESPONSE_TIME: &str = "osu_response_time";
pub(crate) const USERNAME_CACHE_SIZE: &str = "osu_username_cache_size";

pub(crate) fn init_metrics() {
    describe_histogram!(
        RESPONSE_TIME,
        Unit::Seconds,
        "Response time for requests in seconds"
    );

    #[cfg(feature = "cache")]
    metrics::describe_counter!(
        USERNAME_CACHE_SIZE,
        Unit::Count,
        "Number of cached usernames"
    );
}
