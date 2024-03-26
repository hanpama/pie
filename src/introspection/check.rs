use postgres::{Error, Transaction};

struct Check {
    schema: String,
    table: String,
    name: String,
    check_clause: String,
    is_deferrable: bool,
    initially_deferred: bool,
}

fn introspect_check_constraints(
    client: &mut Transaction,
    schemas: &[&str],
) -> Result<Vec<Check>, Error> {
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
    use super::introspect_check_constraints;
    use crate::util::test::get_test_connection;

    #[test]
    fn test_introspect_check_constraints() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        introspect_check_constraints(&mut tx, &vec!["public"]).unwrap();
    }
}
