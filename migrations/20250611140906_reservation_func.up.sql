-- Add up migration script here

CREATE OR REPLACE FUNCTION rsvp.query(user_id text, resource_id text, duration TSTZRANGE) RETURNS TABLE (LIKE rsvp.reservations)
AS $$
BEGIN
    IF user_id IS NULL AND resource_id IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE timespan && duration;
    ELSIF user_id IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE resource_id = resource_id AND duration @> timespan;
    ELSIF  resource_id IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE user_id = user_id AND duration @> timespan;
    ELSE
        RETURN QUERY SELECT * FROM rsvp.reservations WHERE user_id = user_id AND resource_id = resource_id AND duration @> timespan;
    END IF;
END;
$$ LANGUAGE plpgsql;
