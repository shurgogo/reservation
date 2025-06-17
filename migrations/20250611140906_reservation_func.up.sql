-- Add up migration script here

CREATE OR REPLACE FUNCTION rsvp.query(
    user_id text,
    resource_id text,
    duration TSTZRANGE,
    status rsvp.reservation_status,
    page integer DEFAULT 1,
    is_desc bool DEFAULT FALSE,
    -- page size 出现几率小于 desc 所以放在下面
    page_size integer DEFAULT 10
) RETURNS TABLE (LIKE rsvp.reservations) AS $$
DECLARE
    _sql text;
BEGIN
    IF page < 1 THEN
        page := 1;
    END IF;
    IF page_size < 10 OR page_size > 100 THEN
        page_size := 10;
    END IF;
    -- format the query based on the input parameters
    _sql := format(
        'SELECT * FROM rsvp.reservations WHERE %L @> timespan AND status = %L AND %s ORDER BY lower(timespan) %s LIMIT %s OFFSET %s',
        duration,
        status,
        CASE
            WHEN user_id IS NULL AND resource_id IS NULL THEN 'TRUE'
            -- 使用 quote_literal 保证 id 是字符串，可以防止SQL注入
            WHEN user_id IS NULL THEN 'resource_id = ' || quote_literal(resource_id)
            WHEN resource_id IS NULL THEN 'user_id = ' || quote_literal(user_id)
            ELSE 'user_id = ' || quote_literal(user_id) || ' AND resource_id = ' || quote_literal(resource_id)
        END,
        CASE
            WHEN is_desc THEN 'DESC'
            ELSE 'ASC'
        END,
        page_size,
        (page - 1) * page_size
    );

    --log the _sql
    RAISE NOTICE '%', _sql;

    -- execute the query
    RETURN QUERY EXECUTE _sql;
END;
$$ LANGUAGE plpgsql;
