mod pb;

use chrono::{DateTime, Utc};
pub use pb::*;
use prost_types::Timestamp;

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(ts.seconds, ts.nanos as u32).unwrap_or_else(Utc::now)
}
