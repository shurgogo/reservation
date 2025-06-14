use std::ops::Range;

use chrono::{DateTime, FixedOffset, Utc};

use crate::{Error, Reservation, ReservationStatus, convert_to_timestamp, convert_to_utc_time};

impl Reservation {
    pub fn new_pending(
        uid: impl Into<String>,
        rid: impl Into<String>,
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            id: "".to_string(),
            user_id: uid.into(),
            resource_id: rid.into(),
            start: Some(convert_to_timestamp(start.with_timezone(&Utc))),
            end: Some(convert_to_timestamp(end.with_timezone(&Utc))),
            status: ReservationStatus::Pending as i32,
            note: note.into(),
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId("".to_string()));
        }
        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId("".to_string()));
        }
        if self.start.is_none() || self.end.is_none() {
            return Err(Error::InvalidTime);
        }

        let start = convert_to_utc_time(*self.start.as_ref().unwrap());
        let end = convert_to_utc_time(*self.end.as_ref().unwrap());
        if start >= end {
            return Err(Error::InvalidTime);
        }

        Ok(())
    }

    pub fn get_timespan(&self) -> Range<DateTime<Utc>> {
        Range {
            start: convert_to_utc_time(*self.start.as_ref().unwrap()),
            end: convert_to_utc_time(*self.end.as_ref().unwrap()),
        }
    }
}
