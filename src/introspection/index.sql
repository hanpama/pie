SELECT
    pn.nspname
        AS schema_name,
    pc_index.relname
        AS index_name,
    pc_table.relname
        AS table_name,
    pi.indisunique
        AS unique,
    am.amname
        AS method,
    pi.indnatts
        AS key_length,
    (
        SELECT array_agg(pa.attname) 
        FROM unnest(pi.indkey) AS key(i)
            JOIN pg_catalog.pg_attribute pa
                ON pa.attrelid = pi.indrelid
                    AND pa.attnum = i 
    ) AS key_columns,
    ( 
        SELECT array_agg(o)
        FROM unnest(pi.indoption) AS ko(o)
    ) AS key_options,
    pg_catalog.pg_get_expr(pi.indexprs, pi.indrelid)
        AS expressions
FROM pg_catalog.pg_index pi
    JOIN pg_catalog.pg_class pc_index ON pc_index.oid = pi.indexrelid
    JOIN pg_catalog.pg_namespace pn ON pc_index.relnamespace = pn.oid
    JOIN pg_catalog.pg_class pc_table ON pc_table.oid = pi.indrelid
    JOIN pg_am am ON am.oid=pc_index.relam
    LEFT JOIN pg_catalog.pg_constraint pc ON pc.conindid = pc_index.oid
WHERE pc.oid IS NULL
    AND pn.nspname = ANY($1);
