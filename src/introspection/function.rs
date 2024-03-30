use postgres::{Error, Transaction};

#[derive(PartialEq, Debug)]
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
    use crate::{introspection::function::Function, util::test::get_test_connection};

    #[test]
    fn test_introspect_functions() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_function", &[]).unwrap();
        tx.execute(
            "CREATE FUNCTION test_function.func1()
            RETURNS integer
            LANGUAGE sql
            AS $$SELECT 1;$$;",
            &[],
        ).unwrap();
        let res = introspect_functions(&mut tx, &vec!["test_function"]).unwrap();

        assert_eq!(res, vec![
            Function {
                schema: "test_function".to_string(),
                name: "func1".to_string(),
                body: "SELECT 1;".to_string(),
                language: "sql".to_string(),
                returns: "integer".to_string(),
                volatility: "VOLATILE".to_string(),
            },
        ]);
    }
}
