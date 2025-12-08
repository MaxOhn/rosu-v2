use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tracing_subscriber::{fmt::TestWriter, EnvFilter};

pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_writer(TestWriter::new())
        .with_env_filter(EnvFilter::builder().parse("rosu_v2=trace,info").unwrap())
        .try_init();
}

pub fn jitter() -> Duration {
    const MAX_JITTER_MS: u64 = 5000;
    const RANDOM_PRIME: u64 = 1_442_695_040_888_963_407;
    const MS_PER_NS: u128 = 100_000;

    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let jitter = ((ns / MS_PER_NS) as u64).wrapping_mul(RANDOM_PRIME) % MAX_JITTER_MS;

    Duration::from_millis(500 + jitter)
}
