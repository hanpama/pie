use postgres::{Error, Transaction};

pub struct Function {
    pub schema: String,
    pub name: String,
    pub body: String,
    pub language: String,
    pub returns: String,
    pub volatility: String,
}

pub fn introspect_functions(
    client: &mut Transaction,
    schemas: &[&str],
) -> Result<Vec<Function>, Error> {
    let query = include_str!("function.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = Function {
            schema: row.get(0),
            name: row.get(1),
            body: row.get(2),
            language: row.get(3),
            returns: row.get(4),
            volatility: row.get(5),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_functions;
    use crate::util::test::get_test_connection;

    #[test]
    fn test_introspect_functions() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        introspect_functions(&mut tx, &vec!["pg_catalog"]).unwrap();
    }
}
