-- Add down migration script here


DROP TABLE rsvp.reservations CASCADE;
DROP TYPE reservation_update_type;
DROP TYPE reservation_status;

-- indeices will be dropped automatically after  table drop
