SELECT
    ncon.nspname AS constraint_schema,
    con.conname AS constraint_name,
    c.relname AS constraint_table_name,
    (
			SELECT array_agg(pa.attname)
			FROM UNNEST(con.conkey) ck(attnum)
			JOIN pg_catalog.pg_attribute pa ON pa.attrelid = con.conrelid AND pa.attnum = ck.attnum
		) AS table_columns,

    npkc.nspname AS target_schema,
    fc.relname AS target_table_name,
	(
		SELECT array_agg(pa.attname)
		FROM pg_catalog.pg_attribute pa
		WHERE fc.oid = pa.attrelid
		AND fc.oid =
			CASE pkc.contype
				WHEN 'f' THEN pkc.confrelid
				ELSE pkc.conrelid
			END AND (pa.attnum = ANY (
			CASE pkc.contype
				WHEN 'f' THEN pkc.confkey
			ELSE pkc.conkey
		END)) AND NOT pa.attisdropped 
	) AS target_columns,
	con.condeferrable AS constraint_deferrable,
	con.condeferred AS constraint_deferred,
    CASE con.confmatchtype
        WHEN 'f'::"char" THEN 'FULL'
        WHEN 'p'::"char" THEN 'PARTIAL'
        WHEN 's'::"char" THEN 'NONE'
        ELSE NULL
    END::information_schema.character_data AS match_option,
    CASE con.confupdtype
        WHEN 'c'::"char" THEN 'CASCADE'
        WHEN 'n'::"char" THEN 'SET NULL'
        WHEN 'd'::"char" THEN 'SET DEFAULT'
        WHEN 'r'::"char" THEN 'RESTRICT'
        WHEN 'a'::"char" THEN 'NO ACTION'
        ELSE NULL
    END::information_schema.character_data AS update_rule,
        CASE con.confdeltype
            WHEN 'c'::"char" THEN 'CASCADE'
            WHEN 'n'::"char" THEN 'SET NULL'
            WHEN 'd'::"char" THEN 'SET DEFAULT'
            WHEN 'r'::"char" THEN 'RESTRICT'
            WHEN 'a'::"char" THEN 'NO ACTION'
            ELSE NULL
        END::information_schema.character_data AS delete_rule
FROM pg_namespace ncon
    JOIN pg_constraint con ON ncon.oid = con.connamespace
    JOIN pg_class c ON con.conrelid = c.oid AND con.contype = 'f'::"char"
    LEFT JOIN pg_depend d1 ON d1.objid = con.oid AND d1.classid = 'pg_constraint'::regclass::oid AND d1.refclassid = 'pg_class'::regclass::oid AND d1.refobjsubid = 0
    LEFT JOIN pg_depend d2 ON d2.refclassid = 'pg_constraint'::regclass::oid AND d2.classid = 'pg_class'::regclass::oid AND d2.objid = d1.refobjid AND d2.objsubid = 0 AND d2.deptype = 'i'::"char"
    LEFT JOIN pg_class fc ON con.confrelid = fc.oid
    LEFT JOIN pg_constraint pkc ON pkc.oid = d2.refobjid AND (pkc.contype = ANY (ARRAY['p'::"char", 'u'::"char"])) AND pkc.conrelid = con.confrelid
    LEFT JOIN pg_namespace npkc ON pkc.connamespace = npkc.oid
WHERE (pg_has_role(c.relowner, 'USAGE') OR has_table_privilege(c.oid, 'INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES, TRIGGER') OR has_any_column_privilege(c.oid, 'INSERT, UPDATE, REFERENCES'))
  AND ncon.nspname = ANY($1);
