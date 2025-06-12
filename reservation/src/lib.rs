mod error;
mod manager;

pub use error::ReservationError;
use sqlx::PgPool;

pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;

// interact with the database asynchronously
#[async_trait::async_trait]
pub trait Rsvp {
    /// make a reservation
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, ReservationError>;
    /// change reservation status
    async fn change_status(&self, id: ReservationId) -> Result<abi::Reservation, ReservationError>;
    /// update note
    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<abi::Reservation, ReservationError>;
    /// delete reservation
    async fn delete(&self, id: ReservationId) -> Result<(), ReservationError>;
    /// query reservations
    async fn query(
        &self,
        query: abi::ReservationQuery,
    ) -> Result<Vec<abi::Reservation>, ReservationError>;
}

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}
