SELECT
    nc.nspname AS constraint_schema,
    c.conname AS constraint_name,
    nr.nspname AS table_schema,
    r.relname AS table_name,
    (
        SELECT array_agg(pa.attname)
        FROM UNNEST(c.conkey) ck(attnum)
        JOIN pg_catalog.pg_attribute pa ON pa.attrelid = c.conrelid AND pa.attnum = ck.attnum
    ) AS table_columns,
    c.condeferrable
        is_deferrable,
    c.condeferred
        initially_deferred
FROM pg_namespace nc,
    pg_namespace nr,
    pg_constraint c,
    pg_class r
WHERE nc.oid = c.connamespace
    AND nr.oid = r.relnamespace
    AND c.conrelid = r.oid
    AND c.contype = 'p'
    AND (r.relkind = ANY (ARRAY['r'::"char", 'p'::"char"]))
    AND NOT pg_is_other_temp_schema(nr.oid)
    AND (
        pg_has_role(r.relowner, 'USAGE'::text)
        OR has_table_privilege(r.oid, 'INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'::text)
        OR has_any_column_privilege(r.oid, 'INSERT, UPDATE, REFERENCES'::text)
    )
    AND nc.nspname = ANY($1);
