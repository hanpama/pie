use postgres::{Error, Transaction};

pub struct Index {
    pub schema_name: String,
    pub index_name: String,
    pub table_name: String,
    pub unique: bool,
    pub method: String,
    pub key_length: i32,
    pub key_columns: Vec<String>,
    pub key_options: Vec<i64>,
    pub expressions: Option<String>,
}

pub fn introspect_indexes(client: &mut Transaction, schemas: &[&str]) -> Result<Vec<Index>, Error> {
    let query = include_str!("index.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = Index {
            schema_name: row.get(0),
            index_name: row.get(1),
            table_name: row.get(2),
            unique: row.get(3),
            method: row.get(4),
            key_length: row.get(5),
            key_columns: row.get(6),
            key_options: row.get(7),
            expressions: row.get(8),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_indexes;
    use crate::util::test::get_test_connection;

    #[test]
    fn test_introspect_indexes() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        introspect_indexes(&mut tx, &vec!["public"]).unwrap();
    }
}
