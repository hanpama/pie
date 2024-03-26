SELECT
    rs.nspname
        constraint_schema,
    c.relname
        table_name,
    con.conname
        constraint_name,
    "substring"(pg_get_constraintdef(con.oid), 7)
        check_clause,
    con.condeferrable
        is_deferrable,
    con.condeferred
        initially_deferred
FROM pg_constraint con
    LEFT JOIN pg_namespace rs ON rs.oid = con.connamespace
    LEFT JOIN pg_class c ON c.oid = con.conrelid
    LEFT JOIN pg_type t ON t.oid = con.contypid
WHERE pg_has_role(COALESCE(c.relowner, t.typowner), 'USAGE'::text) AND con.contype = 'c'::"char"
    AND con.conrelid != 0
    AND rs.nspname = ANY($1);
