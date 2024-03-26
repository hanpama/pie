SELECT
    nc.nspname
        AS "schema",
    c.relname
        AS "name"
FROM pg_namespace nc
    JOIN pg_class c ON nc.oid = c.relnamespace
    LEFT JOIN (pg_type t
    JOIN pg_namespace nt ON t.typnamespace = nt.oid) ON c.reloftype = t.oid
WHERE c.relkind = ANY (ARRAY['r'::"char", 'p'::"char"])
    AND NOT pg_is_other_temp_schema(nc.oid)
    AND (
        pg_has_role(c.relowner, 'USAGE'::text)
        OR has_table_privilege(c.oid, 'SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'::text)
        OR has_any_column_privilege(c.oid, 'SELECT, INSERT, UPDATE, REFERENCES'::text)
    )
    AND nc.nspname = ANY($1);
