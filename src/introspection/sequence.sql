SELECT
    pn.nspname
        AS schema_name,
    pc.relname 
        AS name,
    (
        SELECT pt.typname 
        FROM pg_catalog.pg_type pt
        WHERE pt.oid = ps.seqtypid
    ) as data_type,
    ps.seqincrement
        AS increment,
    ps.seqmin
        AS min_value,
    ps.seqmax
        AS max_value,
    ps.seqstart
        AS start,
    ps.seqcache
        AS cache,
    ps.seqcycle
        AS cycle,
    (
        SELECT pc.relname
        FROM pg_catalog.pg_class pc
        WHERE pa.attrelid = pc.oid
    ) AS owned_by_table_name,
    pa.attname
        AS owned_by_column_name
FROM pg_catalog.pg_sequence ps
JOIN pg_catalog.pg_class pc 
    ON ps.seqrelid = pc.oid
JOIN pg_catalog.pg_namespace pn
    ON pn."oid" = pc.relnamespace
LEFT JOIN (
    pg_catalog.pg_depend pd
        JOIN pg_catalog.pg_attribute pa
            ON pa.attrelid = pd.refobjid
                AND pa.attnum = pd.refobjsubid 
) ON pd.objid = ps.seqrelid
WHERE pn.nspname = ANY($1);
