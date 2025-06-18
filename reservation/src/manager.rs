use abi::{ReservationId, ReservationQuery, ReservationStatus, Validator};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Row;
use sqlx::postgres::types::PgRange;

use crate::{ReservationManager, Rsvp};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        rsvp.validate()?;
        let mut rsvp_clone = rsvp.clone();

        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timespan();
        let status = ReservationStatus::try_from(rsvp.status).unwrap_or(ReservationStatus::Pending);

        // generate an insert sql for the reservation
        let id = sqlx::query(
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
        rsvp_clone.id = id;
        Ok(rsvp_clone)
    }

    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        // if current status is pending, change it to confirmed, otherwise do nothing
        id.validate()?;
        let rsvp: abi::Reservation = sqlx::query_as(
            "UPDATE rsvp.reservations SET status = 'confirmed' WHERE id = $1 AND status = 'pending' RETURNING *"
        ).bind(id).fetch_one(&self.pool).await?;

        Ok(rsvp)
    }

    async fn update_note(
        &self,
        id: crate::ReservationId,
        note: String,
    ) -> Result<abi::Reservation, abi::Error> {
        id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("UPDATE rsvp.reservations SET note = $1 WHERE id = $2 RETURNING *")
                .bind(note)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;
        Ok(rsvp)
    }

    async fn get(&self, id: ReservationId) -> Result<abi::Reservation, abi::Error> {
        id.validate()?;
        let rsvp: abi::Reservation =
            sqlx::query_as("SELECT * FROM rsvp.reservations WHERE id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(rsvp)
    }

    async fn delete(&self, id: crate::ReservationId) -> Result<(), abi::Error> {
        id.validate()?;
        let rsvp = sqlx::query_as("DELETE FROM rsvp.reservations WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(rsvp)
    }

    async fn query(&self, query: ReservationQuery) -> Result<Vec<abi::Reservation>, abi::Error> {
        let user_id = str_to_option(&query.user_id);
        let resource_id = str_to_option(&query.resource_id);
        let range = query.get_timespan();
        let status =
            ReservationStatus::try_from(query.status).unwrap_or(ReservationStatus::Pending);
        let rsvps = sqlx::query_as(
            "SELECT * FROM rsvp.query($1, $2, $3, $4::rsvp.reservation_status, $5, $6, $7)",
        )
        .bind(user_id)
        .bind(resource_id)
        .bind(range)
        .bind(status.to_string())
        .bind(query.page)
        .bind(query.desc)
        .bind(query.page_size)
        .fetch_all(&self.pool)
        .await?;

        Ok(rsvps)
    }
}

impl ReservationManager {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

fn str_to_option(s: &str) -> Option<&str> {
    if s.is_empty() { None } else { Some(s) }
}
#[cfg(test)]
mod tests {
    use abi::{
        Reservation, ReservationConflict, ReservationConflictInfo, ReservationQueryBuilder,
        ReservationStatus, ReservationWindow,
    };
    use prost_types::Timestamp;
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_should_work_for_valid_window(pool: PgPool) {
        let (rsvp, _) = make_shur_reservation(pool).await;
        assert!(rsvp.id != 0);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_conflict_reservation_should_reject(pool: PgPool) {
        let (_, manager) = make_shur_reservation(pool).await;
        let someone = Reservation::new_pending(
            "somebody",
            "ocean-view-room-777",
            "2025-05-14T15:00:00-0700".parse().unwrap(),
            "2025-05-16T12:00:00-0700".parse().unwrap(),
            "test2",
        );

        let err = manager.reserve(someone).await.unwrap_err();

        let info = ReservationConflictInfo::Parsed(ReservationConflict {
            new: ReservationWindow {
                resource_id: "ocean-view-room-777".to_string(),
                start: "2025-05-14T15:00:00-0700".parse().unwrap(),
                end: "2025-05-16T12:00:00-0700".parse().unwrap(),
            },
            old: ReservationWindow {
                resource_id: "ocean-view-room-777".to_string(),
                start: "2025-05-13T15:00:00-0700".parse().unwrap(),
                end: "2025-05-15T12:00:00-0700".parse().unwrap(),
            },
        });

        assert_eq!(err, abi::Error::ConflictReservation(info))
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_change_status_should_work(pool: PgPool) {
        let (rsvp, manager) = make_shur_reservation(pool).await;
        let rsvp = manager.change_status(rsvp.id).await.unwrap();
        assert_eq!(rsvp.status, ReservationStatus::Confirmed as i32);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn reserve_change_status_not_pending_should_do_nothging(pool: PgPool) {
        let (rsvp, manager) = make_shur_reservation(pool).await;
        let rsvp = manager.change_status(rsvp.id).await.unwrap();

        // change status again should do nothing
        let ret = manager.change_status(rsvp.id).await.unwrap_err();
        assert_eq!(ret, abi::Error::NotFound);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn update_note_should_work(pool: PgPool) {
        let (rsvp, manager) = make_shur_reservation(pool).await;
        let rsvp = manager
            .update_note(rsvp.id, "update note".into())
            .await
            .unwrap();

        assert_eq!(rsvp.note, "update note");
    }
    #[sqlx::test(migrations = "../migrations")]
    async fn get_reservation_should_work(pool: PgPool) {
        let (rsvp1, manager) = make_shur_reservation(pool).await;
        let rsvp2 = manager.get(rsvp1.id).await.unwrap();

        assert_eq!(rsvp1, rsvp2);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn delete_reservation_should_work(pool: PgPool) {
        let (rsvp, manager) = make_shur_reservation(pool).await;
        manager.delete(rsvp.id).await.unwrap();
        let rsvp1 = manager.get(rsvp.id).await.unwrap_err();
        assert_eq!(rsvp1, abi::Error::NotFound);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn query_reservations_should_work(pool: PgPool) {
        let (rsvp, manager) = make_shur_reservation(pool).await;
        let query = ReservationQueryBuilder::default()
            .user_id("shurid")
            .start("2025-05-12T15:00:00-0700".parse::<Timestamp>().unwrap())
            .end("2025-05-15T12:00:00-0700".parse::<Timestamp>().unwrap())
            .status(ReservationStatus::Pending as i32)
            .build()
            .unwrap();

        let rsvps = manager.query(query).await.unwrap();
        assert_eq!(rsvps.len(), 1);
        assert_eq!(rsvps[0], rsvp);

        let query = ReservationQueryBuilder::default()
            .user_id("shurid")
            .start("2025-05-13T16:00:00-0700".parse::<Timestamp>().unwrap())
            .end("2025-05-15T12:00:00-0700".parse::<Timestamp>().unwrap())
            .status(ReservationStatus::Pending as i32)
            .build()
            .unwrap();
        let rsvps = manager.query(query).await.unwrap();
        assert!(rsvps.is_empty());
    }

    async fn make_shur_reservation(pool: PgPool) -> (Reservation, ReservationManager) {
        make_reservation(
            pool,
            "shurid",
            "ocean-view-room-777",
            "2025-05-13T15:00:00-0700",
            "2025-05-15T12:00:00-0700",
            "this is shur's reservation",
        )
        .await
    }

    async fn make_reservation(
        pool: PgPool,
        user_id: &str,
        resource_id: &str,
        start: &str,
        end: &str,
        note: &str,
    ) -> (Reservation, ReservationManager) {
        let manager = ReservationManager::new(pool);
        let rsvp = Reservation::new_pending(
            user_id,
            resource_id,
            start.parse().unwrap(),
            end.parse().unwrap(),
            note,
        );
        let rsvp = manager.reserve(rsvp).await.unwrap();
        (rsvp, manager)
    }
}
