SELECT 
    nc.nspname 
        AS "schema",
    c.relname
        AS "table",
    a.attnum
        AS "ordinal_position",
    a.attname
        AS "column_name",
    COALESCE( CASE WHEN a.attgenerated = '' THEN pg_get_expr(ad.adbin, ad.adrelid) ELSE null END, '')
        AS "column_default",
    a.attnotnull OR t.typtype = 'd' AND t.typnotnull
        AS "not_null",
    CASE
        WHEN t.typelem <> 0 THEN (SELECT eltype.typname FROM pg_type eltype WHERE eltype.oid = t.typelem) 
        ELSE t.typname
    END 
    ||
    CASE WHEN a.atttypmod != -1 THEN
        CASE t.typname
                WHEN 'bpchar' THEN bpchartypmodout(a.atttypmod)
                WHEN '_bpchar' THEN bpchartypmodout(a.atttypmod)
                WHEN 'varchar' THEN varchartypmodout(a.atttypmod)
                WHEN '_varchar' THEN varchartypmodout(a.atttypmod)
                WHEN 'time' THEN timetypmodout(a.atttypmod)
                WHEN '_time' THEN timetypmodout(a.atttypmod)
                WHEN 'timestamp' THEN timestamptypmodout(a.atttypmod)
                WHEN '_timestamp' THEN timestamptypmodout(a.atttypmod)
                WHEN 'timestamptz' THEN timestamptztypmodout(a.atttypmod)
                WHEN '_timestamptz' THEN timestamptztypmodout(a.atttypmod)
                WHEN 'interval' THEN intervaltypmodout(a.atttypmod)
                WHEN '_interval' THEN intervaltypmodout(a.atttypmod)
                WHEN 'timetz' THEN timetztypmodout(a.atttypmod)
                WHEN '_timetz' THEN timetztypmodout(a.atttypmod)
                WHEN 'bit' THEN bittypmodout(a.atttypmod)
                WHEN '_bit' THEN bittypmodout(a.atttypmod)
                WHEN 'varbit' THEN varbittypmodout(a.atttypmod)
                WHEN '_varbit' THEN varbittypmodout(a.atttypmod)
                WHEN 'numeric' THEN numerictypmodout(a.atttypmod)
                WHEN '_numeric' THEN numerictypmodout(a.atttypmod)
                ELSE null
        END::TEXT
    ELSE ''
    END
    ||
    CASE WHEN t.typelem <> 0 THEN '[]' ELSE '' END
        AS "data_type"
FROM pg_attribute a
    LEFT JOIN pg_attrdef ad ON a.attrelid = ad.adrelid AND a.attnum = ad.adnum
    JOIN (pg_class c JOIN pg_namespace nc ON c.relnamespace = nc.oid) ON a.attrelid = c.oid
    JOIN (pg_type t JOIN pg_namespace nt ON t.typnamespace = nt.oid) ON a.atttypid = t.oid
WHERE NOT pg_is_other_temp_schema(nc.oid)
    AND a.attnum > 0
    AND NOT a.attisdropped
    AND c.relkind = 'r'
    AND nc.nspname = ANY($1);
