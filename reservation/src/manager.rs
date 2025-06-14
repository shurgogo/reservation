use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;
use sqlx::{Row, types::Uuid};

use crate::{ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        rsvp.validate()?;
        let mut rsvp_clone = rsvp.clone();

        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timespan().into();
        let status = abi::ReservationStatus::try_from(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        // generate an insert sql for the reservation
        let id: Uuid = sqlx::query(
            "INSERT INTO rsvp.reservations
                 (user_id, resource_id, timespan, note, status)
                 VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status)
                 RETURNING id",
        )
        .bind(rsvp.user_id)
        .bind(rsvp.resource_id)
        .bind(timespan)
        .bind(rsvp.note)
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?
        .get(0);
        rsvp_clone.id = id.to_string();
        Ok(rsvp_clone)
    }

    async fn change_status(
        &self,
        _id: crate::ReservationId,
    ) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }

    async fn update_note(
        &self,
        _id: crate::ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, abi::Error> {
        todo!()
    }

    async fn delete(&self, _id: crate::ReservationId) -> Result<(), abi::Error> {
        todo!()
    }

    async fn query(
        &self,
        _query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, abi::Error> {
        todo!()
    }
}

impl ReservationManager {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use abi::ReservationConflictInfo;
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_should_work_for_valid_window(pool: PgPool) {
        let manager = ReservationManager::new(pool);
        let rsvp = abi::Reservation::new_pending(
            "userid",
            "resourceid",
            "2025-05-13T15:00:00-0700".parse().unwrap(),
            "2025-05-14T12:00:00-0700".parse().unwrap(),
            "test",
        );
        let rsvp = manager.reserve(rsvp).await.unwrap();
        assert!(!rsvp.id.is_empty());
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_conflict_reservation_should_reject(pool: PgPool) {
        let manager = ReservationManager::new(pool);
        let rsvp1 = abi::Reservation::new_pending(
            "shurid",
            "ocean-view-room-777",
            "2025-05-13T15:00:00-0700".parse().unwrap(),
            "2025-05-15T12:00:00-0700".parse().unwrap(),
            "test2",
        );
        let rsvp2 = abi::Reservation::new_pending(
            "anotherid",
            "ocean-view-room-777",
            "2025-05-14T15:00:00-0700".parse().unwrap(),
            "2025-05-16T12:00:00-0700".parse().unwrap(),
            "test2",
        );

        let _rsvp1 = manager.reserve(rsvp1).await.unwrap();
        let err = manager.reserve(rsvp2).await.unwrap_err();

        if let abi::Error::ConflictReservation(ReservationConflictInfo::Parsed(conflict)) = err {
            assert_eq!(conflict.new.resource_id, "ocean-view-room-777");
            assert_eq!(conflict.new.start.to_rfc3339(), "2025-05-14T22:00:00+00:00");
            assert_eq!(conflict.new.end.to_rfc3339(), "2025-05-16T19:00:00+00:00");
            assert_eq!(conflict.old.resource_id, "ocean-view-room-777");
            assert_eq!(conflict.old.start.to_rfc3339(), "2025-05-13T22:00:00+00:00");
            assert_eq!(conflict.old.end.to_rfc3339(), "2025-05-15T19:00:00+00:00");
        } else {
            panic!("expected ConflictReservation error");
        }
    }
}
