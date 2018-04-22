use std::time::Duration;

pub fn duration_total_ms(dur: Duration) -> f64 {
    let total_secs = dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1_000_000_000.0;
    total_secs * 1000.0
}