use postgres::{Client, Error};

struct CheckConstraint {
    schema: String,
    table: String,
    name: String,
    check_clause: String,
    is_deferrable: bool,
    initially_deferred: bool,
    description: String,
}

fn introspect_check_constraints(
    client: &mut Client,
    schemas: Vec<String>,
) -> Result<Vec<CheckConstraint>, Error> {
    let query = include_str!("check.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = CheckConstraint {
            schema: row.get(0),
            table: row.get(1),
            name: row.get(2),
            check_clause: row.get(3),
            is_deferrable: row.get(4),
            initially_deferred: row.get(5),
            description: row.get(6),
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
        introspect_check_constraints(&mut conn, vec!["public".to_owned()]).unwrap();
    }
}
