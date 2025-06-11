-- Add down migration script here

DROP EXTENSION btree_gist;
DROP SCHEMA rsvp CASCADE;

-- TODO: consider creating a role for the application
