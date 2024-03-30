use postgres::{Error, Transaction};

#[derive(PartialEq, Debug)]
pub struct Sequence {
    pub schema: String,
    pub name: String,
    pub data_type: String,
    pub increment: i64,
    pub min_value: i64,
    pub max_value: i64,
    pub start: i64,
    pub cache: i64,
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
        tx.execute("CREATE SCHEMA test_sequence", &[]).unwrap();
        tx.execute(
            "CREATE SEQUENCE test_sequence.sequence1
            AS int4
            INCREMENT BY 1
            MINVALUE 1
            MAXVALUE 100
            START WITH 1
            CACHE 1
            CYCLE",
            &[],
        )
        .unwrap();
        let res = introspect_sequences(&mut tx, &vec!["test_sequence"]).unwrap();

        assert_eq!(
            res,
            vec![crate::introspection::sequence::Sequence {
                schema: "test_sequence".to_string(),
                name: "sequence1".to_string(),
                data_type: "int4".to_string(),
                increment: 1,
                min_value: 1,
                max_value: 100,
                start: 1,
                cache: 1,
                cycle: true,
                owned_by_table_name: None,
                owned_by_column_name: None,
            }]
        );
    }
}
