use postgres::{Error, Transaction};

pub struct Sequence {
    pub schema: String,
    pub name: String,
    pub data_type: String,
    pub increment: i64,
    pub min_value: i64,
    pub max_value: i64,
    pub start: i64,
    pub cache: i32,
    pub cycle: bool,
    pub owned_by_table_name: Option<String>,
    pub owned_by_column_name: Option<String>,
}

pub fn introspect_sequences(
    client: &mut Transaction,
    schemas: &[&str],
) -> Result<Vec<Sequence>, Error> {
    let query = include_str!("sequence.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = Sequence {
            schema: row.get(0),
            name: row.get(1),
            data_type: row.get(2),
            increment: row.get(3),
            min_value: row.get(4),
            max_value: row.get(5),
            start: row.get(6),
            cache: row.get(7),
            cycle: row.get(8),
            owned_by_table_name: row.get(9),
            owned_by_column_name: row.get(10),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_sequences;
    use crate::util::test::get_test_connection;

    #[test]
    fn test_introspect_sequences() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        introspect_sequences(&mut tx, &vec!["public"]).unwrap();
    }
}
