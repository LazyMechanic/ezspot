use chrono::{Duration, NaiveDateTime, Utc};

pub fn expires_timestamp(sec_duration: i64) -> NaiveDateTime {
    (Utc::now() + Duration::seconds(sec_duration)).naive_utc()
}
