use chrono::{Duration, Utc};

pub fn expires_timestamp(sec_duration: i64) -> i64 {
    (Utc::now() + Duration::seconds(sec_duration)).timestamp()
}
