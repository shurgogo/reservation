use std::ops::Bound;

use chrono::{DateTime, FixedOffset, Utc};
use sqlx::{
    FromRow, Row,
    postgres::{PgRow, types::PgRange},
};

use crate::{
    Error, Reservation, ReservationStatus, RsvpStatus, Validator, convert_to_timestamp,
    types::{get_timespan, validate_range},
};

impl Reservation {
    pub fn new_pending(
        user_id: impl Into<String>,
        resource_id: impl Into<String>,
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            id: 0,
            user_id: user_id.into(),
            resource_id: resource_id.into(),
            start: Some(convert_to_timestamp(start.with_timezone(&Utc))),
            end: Some(convert_to_timestamp(end.with_timezone(&Utc))),
            status: ReservationStatus::Pending as i32,
            note: note.into(),
        }
    }

    pub fn get_timespan(&self) -> PgRange<DateTime<Utc>> {
        get_timespan(self.start.as_ref(), self.end.as_ref())
    }
}

impl Validator for Reservation {
    fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId("".to_string()));
        }
        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId("".to_string()));
        }
        if self.start.is_none() || self.end.is_none() {
            return Err(Error::InvalidTime);
        }
        validate_range(self.start.as_ref(), self.end.as_ref())?;
        Ok(())
    }
}
//impl Id for Reservation {
//    fn id(&self) -> i64 {
//        self.id
//    }
//}

impl FromRow<'_, PgRow> for Reservation {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id = row.get("id");
        let range: PgRange<DateTime<Utc>> = row.get("timespan");
        let range: NaiveRange<DateTime<Utc>> = range.into();

        // in real world, reservation will always have a bound
        assert!(range.start.is_some());
        assert!(range.end.is_some());

        let start = range.start.unwrap();
        let end = range.end.unwrap();

        let status: RsvpStatus = row.get("status");

        Ok(Self {
            id,
            user_id: row.get("user_id"),
            resource_id: row.get("resource_id"),
            start: Some(convert_to_timestamp(start)),
            end: Some(convert_to_timestamp(end)),
            note: row.get("note"),
            status: ReservationStatus::from(status) as i32,
        })
    }
}

struct NaiveRange<T> {
    start: Option<T>,
    end: Option<T>,
}

impl<T> From<PgRange<T>> for NaiveRange<T> {
    fn from(range: PgRange<T>) -> Self {
        let f = |b| match b {
            Bound::Included(v) => Some(v),
            Bound::Excluded(v) => Some(v),
            Bound::Unbounded => None,
        };
        let start = f(range.start);
        let end = f(range.end);
        Self { start, end }
    }
}
