-- Add up migration script here

CREATE SCHEMA rsvp;

-- AWS RDS support
-- https://docs.aws.amazon.com/AmazonRDS/latest/PostgreSQLReleaseNotes/postgresql-extensions.html
CREATE EXTENSION btree_gist;
