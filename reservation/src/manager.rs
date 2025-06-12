use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::postgres::types::PgRange;

use crate::{ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(
        &self,
        rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, crate::ReservationError> {
        let mut rsvp_clone = rsvp.clone();
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(crate::ReservationError::InvalidTime);
        }
        let start = abi::convert_to_utc_time(rsvp.start.unwrap());
        let end = abi::convert_to_utc_time(rsvp.end.unwrap());
        let timespan: PgRange<DateTime<Utc>> = (start..end).into();

        // generate an insert sql for the reservation
        let id = sqlx::query(
            "INSERT INTO rsvp.reservations
                 (user_id, resource_id, timespan, note, status)
                 VALUES ($1, $2, $3, $4, $5)
                 RETURNING id",
        )
        .bind(rsvp.user_id)
        .bind(rsvp.resource_id)
        .bind(timespan)
        .bind(rsvp.note)
        .bind(rsvp.status)
        .fetch_one(&self.pool)
        .await?
        .get(0);
        rsvp_clone.id = id;
        Ok(rsvp_clone)
    }

    async fn change_status(
        &self,
        _id: crate::ReservationId,
    ) -> Result<abi::Reservation, crate::ReservationError> {
        todo!()
    }

    async fn update_note(
        &self,
        _id: crate::ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, crate::ReservationError> {
        todo!()
    }

    async fn delete(&self, _id: crate::ReservationId) -> Result<(), crate::ReservationError> {
        todo!()
    }

    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, crate::ReservationError> {
        todo!()
    }
}
