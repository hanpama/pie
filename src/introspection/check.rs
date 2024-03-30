use postgres::{Error, Transaction};

#[derive(Debug, PartialEq)]
pub struct Check {
    pub schema: String,
    pub table: String,
    pub name: String,
    pub check_clause: String,
    pub is_deferrable: bool,
    pub initially_deferred: bool,
}

pub fn introspect_checks(client: &mut Transaction, schemas: &[&str]) -> Result<Vec<Check>, Error> {
    let query = include_str!("check.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = Check {
            schema: row.get(0),
            table: row.get(1),
            name: row.get(2),
            check_clause: row.get(3),
            is_deferrable: row.get(4),
            initially_deferred: row.get(5),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_checks;
    use crate::{introspection::check::Check, util::test::get_test_connection};

    #[test]
    fn test_introspect_checks() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_check;", &[]).unwrap();
        tx.execute(
            "CREATE TABLE test_check.foo (
                id SERIAL PRIMARY KEY,
                name TEXT,
                CONSTRAINT check_name CHECK (name <> 'foo')
            );",
            &[],
        )
        .unwrap();
        let res = introspect_checks(&mut tx, &vec!["test_check"]).unwrap();

        assert_eq!(
            res,
            vec![Check {
                schema: "test_check".to_string(),
                table: "foo".to_string(),
                name: "check_name".to_string(),
                check_clause: "((name <> 'foo'::text))".to_string(),
                is_deferrable: false,
                initially_deferred: false,
            }]
        );
    }
}
