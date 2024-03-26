SELECT
    nspname
        AS "schema_name"
FROM pg_catalog.pg_namespace pn
WHERE nspname = ANY($1);
