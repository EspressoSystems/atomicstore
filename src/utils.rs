use std::time::SystemTime;

/// Get the unix timestamp
pub fn unix_timestamp() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        // `duration_since` will error if `SystemTime::now()` is earlier than `UNIX_EPOCH`
        // in that case, we just return the negative duration
        Err(e) => -(e.duration().as_secs() as i64),
    }
}
