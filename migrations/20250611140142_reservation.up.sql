-- Add up migration script here

CREATE TYPE rsvp.reservation_status AS ENUM (
    'unknown', 'pending', 'confirmed', 'blocked'
);
CREATE TYPE rsvp.reservation_update_type AS ENUM (
    'unknown', 'create', 'update', 'delete'
);

CREATE TABLE rsvp.reservations (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id VARCHAR(64) NOT NULL,
    status rsvp.reservation_status NOT NULL DEFAULT 'pending',

    resource_id VARCHAR(64) NOT NULL,
    timespan TSTZRANGE NOT NULL,

    note TEXT,

    CONSTRAINT reservations_pkey PRIMARY KEY (id),
    CONSTRAINT reservation_conflict EXCLUDE USING gist (resource_id WITH =, timespan WITH &&)
);

CREATE INDEX idx_reservations_resource_id ON rsvp.reservations USING btree (resource_id);
CREATE INDEX idx_reservations_user_id ON rsvp.reservations USING btree (user_id);
