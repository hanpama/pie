SELECT
    n.nspname AS schema,
    p.proname AS name,
    p.prosrc AS body,
    l.lanname AS language,
    CASE
        WHEN p.proretset THEN 'setof ' || format_type(p.prorettype, NULL)
        ELSE format_type(p.prorettype, NULL)
    END AS returns,
    CASE
        WHEN p.provolatile = 'i' THEN 'IMMUTABLE'
        WHEN p.provolatile = 's' THEN 'STABLE'
        ELSE 'VOLATILE'
    END AS volatility
FROM
    pg_proc p
    JOIN pg_namespace n ON p.pronamespace = n.oid
    JOIN pg_language l ON p.prolang = l.oid
WHERE n.nspname = ANY($1)
