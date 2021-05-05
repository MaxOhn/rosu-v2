use std::time::{Duration, Instant};
use tokio::{sync::Mutex, time::sleep};

struct RatelimitData {
    allowance: f32,
    last_call: Instant,
}

/// Basic rate limiter that grants access for a certain amount of times within a time span.
/// Implemented through token bucket algorithm.
pub(crate) struct Ratelimiter {
    rate: f32,
    rate_per_ms: f32,
    data: Mutex<RatelimitData>,
}

impl Ratelimiter {
    /// Creates a new RateLimiter.
    /// Allows for up to `rate` amount of access calls within `per_seconds` amount of seconds.
    pub(crate) fn new(rate: u32, per_seconds: u32) -> Self {
        Self {
            rate: rate as f32,
            rate_per_ms: rate as f32 / per_seconds as f32 / 1000.0,
            data: Mutex::new(RatelimitData {
                allowance: 0.0,
                last_call: Instant::now(),
            }),
        }
    }

    /// Wait until the next access
    pub(crate) async fn await_access(&self) {
        let mut data = self.data.lock().await;
        let elapsed = data.last_call.elapsed().as_millis() as f32; // ms
        data.allowance += elapsed * self.rate_per_ms; // msgs

        if data.allowance > self.rate {
            data.allowance = self.rate - 1.0;
        } else if data.allowance < 1.0 {
            let ms_left = (1.0 - data.allowance) / self.rate_per_ms; // s
            sleep(Duration::from_micros((1000.0 * ms_left).round() as u64)).await;
            data.allowance = 0.0;
        } else {
            data.allowance -= 1.0;
        }

        data.last_call = Instant::now();
    }
}
