use postgres::{Error, Transaction};

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
    use crate::util::test::get_test_connection;

    #[test]
    fn test_introspect_tables() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        introspect_tables(&mut tx, &vec!["public"]).unwrap();
    }
}
