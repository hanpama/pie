SELECT
    nc.nspname
        AS schema,
    c.relname
        AS name,
    CASE
        WHEN pg_has_role(c.relowner, 'USAGE'::text) THEN left(pg_get_viewdef(c.oid), -1)
        ELSE NULL::text
    END
        AS view_definition,
    CASE
        WHEN 'check_option=cascaded'::text = ANY (c.reloptions) THEN 'CASCADED'::text
        WHEN 'check_option=local'::text = ANY (c.reloptions) THEN 'LOCAL'::text
        ELSE 'NONE'::text
    END
        AS check_option,
    (pg_relation_is_updatable(c.oid::regclass, false) & 20) = 20
        AS is_updatable,
    (pg_relation_is_updatable(c.oid::regclass, false) & 8) = 8
        AS is_insertable_into,
    EXISTS (
        SELECT 1
        FROM pg_trigger
        WHERE pg_trigger.tgrelid = c.oid AND (pg_trigger.tgtype::integer & 81) = 81
    )
        AS is_trigger_updatable,
    EXISTS (
        SELECT 1
        FROM pg_trigger
        WHERE pg_trigger.tgrelid = c.oid AND (pg_trigger.tgtype::integer & 73) = 73
    )
        AS is_trigger_deletable,
    EXISTS (
        SELECT 1
        FROM pg_trigger
        WHERE pg_trigger.tgrelid = c.oid AND (pg_trigger.tgtype::integer & 69) = 69
    ) AS is_trigger_insertable_into
FROM pg_namespace nc,
pg_class c
WHERE nc.nspname = ANY($1)
    AND c.relnamespace = nc.oid
    AND c.relkind = 'v'::"char"
    AND NOT pg_is_other_temp_schema(nc.oid)
    AND (pg_has_role(c.relowner, 'USAGE'::text)
    OR has_table_privilege(c.oid, 'SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER'::text)
    OR has_any_column_privilege(c.oid, 'SELECT, INSERT, UPDATE, REFERENCES'::text));
