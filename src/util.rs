use std::time::Duration;

pub fn f64_to_duration(value: f64) -> Duration {
    let millis = (value * 1_000.0).round() as u64;
    Duration::from_millis(millis)
}
