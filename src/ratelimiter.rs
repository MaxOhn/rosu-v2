use std::time::{Duration, Instant};
use tokio::{sync::Mutex, time::sleep};

/// Basic rate limiter that grants access for a certain amount of times within a time span.
/// Implemented through token bucket algorithm.
pub(crate) struct Ratelimiter {
    rate: f32,
    rate_per_ms: f32,
    allowance: Mutex<f32>,
    last_call: Mutex<Instant>,
}

impl Ratelimiter {
    /// Creates a new RateLimiter.
    /// Allows for up to `rate` amount of access calls within `per_seconds` amount of seconds.
    pub(crate) fn new(rate: u32, per_seconds: u32) -> Self {
        Self {
            rate: rate as f32,
            rate_per_ms: rate as f32 / per_seconds as f32 / 1000.0,
            allowance: Mutex::new(0.0),
            last_call: Mutex::new(Instant::now()),
        }
    }

    /// Wait until the next access
    pub(crate) async fn await_access(&self) {
        let mut allowance = self.allowance.lock().await;
        let elapsed = self.last_call.lock().await.elapsed().as_millis() as f32; // ms
        *allowance += elapsed * self.rate_per_ms; // msgs
        if *allowance > self.rate {
            *allowance = self.rate - 1.0;
        } else if *allowance < 1.0 {
            let ms_left = (1.0 - *allowance) / self.rate_per_ms; // s
            sleep(Duration::from_micros((1000.0 * ms_left).round() as u64)).await;
            *allowance = 0.0;
        } else {
            *allowance -= 1.0;
        }
        *self.last_call.lock().await = Instant::now();
    }
}
