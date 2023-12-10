SELECT 
    nc.nspname AS "schema",
    c.relname AS "table",
    a.attnum AS ordinal_position,
    a.attname AS column_name,
    COALESCE( CASE WHEN a.attgenerated = '' THEN pg_get_expr(ad.adbin, ad.adrelid) ELSE null END, '') AS column_default,
    CASE
            WHEN a.attnotnull
                    OR t.typtype = 'd'
                    AND t.typnotnull THEN TRUE
            ELSE FALSE
    END AS not_null,
    TRIM(LEADING '_' FROM COALESCE(bt.typname, t.typname)) as type_name,
    CASE
            WHEN t.typtype = 'd'
                    AND bt.typelem <> 0
                    AND bt.typlen = -1 THEN TRUE
            WHEN t.typelem <> 0
                    AND t.typlen = -1 THEN TRUE
            ELSE FALSE
    END AS is_array,
    COALESCE(
        CASE a.atttypmod WHEN -1 THEN NULL ELSE a.atttypmod END,
        CASE t.typtypmod WHEN -1 THEN NULL ELSE t.typtypmod END
    ) typmod,
FROM pg_attribute a
    LEFT JOIN pg_attrdef ad ON a.attrelid = ad.adrelid AND a.attnum = ad.adnum
    JOIN (pg_class c JOIN pg_namespace nc ON c.relnamespace = nc.oid) ON a.attrelid = c.oid
    JOIN (pg_type t JOIN pg_namespace nt ON t.typnamespace = nt.oid) ON a.atttypid = t.oid
    LEFT JOIN (pg_type bt JOIN pg_namespace nbt ON bt.typnamespace = nbt.oid) ON t.typtype = 'd' AND t.typbasetype = bt.oid
WHERE NOT pg_is_other_temp_schema(nc.oid)
    AND a.attnum > 0
    AND NOT a.attisdropped
    AND (c.relkind = ANY (ARRAY['r', 'v', 'f', 'p'])) 
    AND (pg_has_role(c.relowner, 'USAGE') OR has_column_privilege(c.oid, a.attnum, 'SELECT, INSERT, UPDATE, REFERENCES'))
    AND nc.nspname = ANY($1);
