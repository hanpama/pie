use postgres::{Error, Transaction};

#[derive(Debug, PartialEq)]
pub struct PrimaryKey {
    pub schema: String,
    pub name: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_columns: Vec<String>,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

pub fn introspect_primary_keys(
    client: &mut Transaction,
    schemas: &[&str],
) -> Result<Vec<PrimaryKey>, Error> {
    let query = include_str!("primarykey.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = PrimaryKey {
            schema: row.get(0),
            name: row.get(1),
            table_schema: row.get(2),
            table_name: row.get(3),
            table_columns: row.get(4),
            deferrable: row.get(5),
            initially_deferred: row.get(6),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_primary_keys;
    use crate::{introspection::primarykey::PrimaryKey, util::test::get_test_connection};

    #[test]
    fn test_introspect_primary_keys() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_primarykey;", &[]).unwrap();
        tx.execute(
            "CREATE TABLE test_primarykey.table1 (
                col1 INT,
                CONSTRAINT table1_pkey PRIMARY KEY (col1)
            );",
            &[],
        )
        .unwrap();
        tx.execute(
            "CREATE TABLE test_primarykey.table2 (
                col1 INT,
                col2 INT,
                CONSTRAINT table2_pkey PRIMARY KEY (col1, col2)
            );",
            &[],
        )
        .unwrap();

        let res = introspect_primary_keys(&mut tx, &vec!["test_primarykey"]).unwrap();

        assert_eq!(
            res,
            vec![
                PrimaryKey {
                    schema: "test_primarykey".to_string(),
                    name: "table1_pkey".to_string(),
                    table_schema: "test_primarykey".to_string(),
                    table_name: "table1".to_string(),
                    table_columns: vec!["col1".to_string()],
                    deferrable: false,
                    initially_deferred: false,
                },
                PrimaryKey {
                    schema: "test_primarykey".to_string(),
                    name: "table2_pkey".to_string(),
                    table_schema: "test_primarykey".to_string(),
                    table_name: "table2".to_string(),
                    table_columns: vec!["col1".to_string(), "col2".to_string()],
                    deferrable: false,
                    initially_deferred: false,
                }
            ]
        );
    }
}
