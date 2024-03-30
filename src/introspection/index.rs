use postgres::{Error, Transaction};

#[derive(Debug, PartialEq)]
pub struct Index {
    pub schema_name: String,
    pub index_name: String,
    pub table_name: String,
    pub unique: bool,
    pub method: String,
    pub key_length: i16,
    pub key_columns: Option<Vec<String>>,
    pub key_options: Vec<i16>,
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
    use crate::{introspection::index::Index, util::test::get_test_connection};

    #[test]
    fn test_introspect_indexes() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_index", &[]).unwrap();
        tx.execute(
            "CREATE TABLE test_index.table1 (
                col1 INT,
                col2 INT
            );",
            &[],
        )
        .unwrap();
        tx.execute(
            "CREATE UNIQUE INDEX idx_table1_1 ON test_index.table1(col1)",
            &[],
        )
        .unwrap();
        tx.execute(
            "CREATE INDEX idx_table1_2 ON test_index.table1(col1 ASC, col2 DESC)",
            &[],
        )
        .unwrap();
        let res = introspect_indexes(&mut tx, &vec!["test_index"]).unwrap();

        assert_eq!(
            res,
            vec![
                Index {
                    schema_name: "test_index".to_string(),
                    index_name: "idx_table1_1".to_string(),
                    table_name: "table1".to_string(),
                    unique: true,
                    method: "btree".to_string(),
                    key_length: 1,
                    key_columns: Some(vec!["col1".to_string(),],),
                    key_options: vec![0,],
                    expressions: None,
                },
                Index {
                    schema_name: "test_index".to_string(),
                    index_name: "idx_table1_2".to_string(),
                    table_name: "table1".to_string(),
                    unique: false,
                    method: "btree".to_string(),
                    key_length: 2,
                    key_columns: Some(vec!["col1".to_string(), "col2".to_string()],),
                    key_options: vec![0, 3,],
                    expressions: None,
                },
            ]
        )
    }
}
