use postgres::{Error, Transaction};

#[derive(Debug, PartialEq)]
pub struct Table {
    pub schema: String,
    pub name: String,
}

pub fn introspect_tables(client: &mut Transaction, schemas: &[&str]) -> Result<Vec<Table>, Error> {
    let query = include_str!("table.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = Table {
            schema: row.get(0),
            name: row.get(1),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_tables;
    use crate::{introspection::table::Table, util::test::get_test_connection};

    #[test]
    fn test_introspect_tables() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_table", &[]).unwrap();
        tx.execute("CREATE TABLE test_table.foo (id SERIAL PRIMARY KEY);", &[])
            .unwrap();
        let res = introspect_tables(&mut tx, &vec!["test_table"]).unwrap();

        assert_eq!(
            res,
            vec![Table {
                schema: "test_table".to_string(),
                name: "foo".to_string(),
            }]
        );
    }
}
