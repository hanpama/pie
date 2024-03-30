use postgres::{Error, Transaction};

#[derive(Debug, PartialEq)]
pub struct Unique {
    pub schema: String,
    pub name: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_columns: Vec<String>,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

pub fn introspect_uniques(
    client: &mut Transaction,
    schemas: &[&str],
) -> Result<Vec<Unique>, Error> {
    let query = include_str!("unique.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = Unique {
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
    use super::introspect_uniques;
    use crate::{introspection::unique::Unique, util::test::get_test_connection};

    #[test]
    fn test_introspect_uniques() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_unique;", &[]).unwrap();
        tx.execute(
            "CREATE TABLE test_unique.table1 (
                col1 INT,
                col2 INT,
                CONSTRAINT table1_unique1 UNIQUE (col1),
                CONSTRAINT table1_unique2 UNIQUE (col1, col2)
            );",
            &[],
        )
        .unwrap();
        let res = introspect_uniques(&mut tx, &vec!["test_unique"]).unwrap();
        assert_eq!(
            res,
            vec![
                Unique {
                    schema: "test_unique".to_string(),
                    name: "table1_unique1".to_string(),
                    table_schema: "test_unique".to_string(),
                    table_name: "table1".to_string(),
                    table_columns: vec!["col1".to_string()],
                    deferrable: false,
                    initially_deferred: false,
                },
                Unique {
                    schema: "test_unique".to_string(),
                    name: "table1_unique2".to_string(),
                    table_schema: "test_unique".to_string(),
                    table_name: "table1".to_string(),
                    table_columns: vec!["col1".to_string(), "col2".to_string()],
                    deferrable: false,
                    initially_deferred: false,
                }
            ]
        );
    }
}
