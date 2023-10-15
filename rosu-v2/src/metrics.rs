#![cfg(feature = "metrics")]

use metrics::{describe_histogram, Unit};

pub(crate) fn init_metrics() {
    describe_histogram!(
        "osu_response_time",
        Unit::Seconds,
        "Response time for requests in seconds"
    );

    #[cfg(feature = "cache")]
    metrics::describe_counter!(
        "osu_username_cache_size",
        Unit::Count,
        "Number of cached usernames"
    );
}
