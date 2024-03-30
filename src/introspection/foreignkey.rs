use postgres::{Error, Transaction};

#[derive(Debug, PartialEq)]
pub struct ForeignKey {
    pub constraint_name: String,
    pub constraint_schema: String,
    pub constraint_table_name: String,
    pub columns: Vec<String>,
    pub target_table_name: String,
    pub target_table_schema: String,
    pub target_table_columns: Vec<String>,
    pub deferrable: bool,
    pub initially_deferred: bool,
    pub match_option: String,
    pub update_rule: String,
    pub delete_rule: String,
}

pub fn introspect_foreign_keys(
    client: &mut Transaction,
    schemas: &[&str],
) -> Result<Vec<ForeignKey>, Error> {
    let query = include_str!("foreignkey.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = ForeignKey {
            constraint_schema: row.get(0),
            constraint_name: row.get(1),
            constraint_table_name: row.get(2),
            columns: row.get(3),
            target_table_schema: row.get(4),
            target_table_name: row.get(5),
            target_table_columns: row.get(6),
            deferrable: row.get(7),
            initially_deferred: row.get(8),
            match_option: row.get(9),
            update_rule: row.get(10),
            delete_rule: row.get(11),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_foreign_keys;
    use crate::util::test::get_test_connection;

    #[test]
    fn test_introspect_foreign_keys() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_foreignkey", &[]).unwrap();
        tx.execute(
            "CREATE TABLE test_foreignkey.table1 (id SERIAL PRIMARY KEY)",
            &[],
        )
        .unwrap();
        tx.execute(
            "CREATE TABLE test_foreignkey.table2 (table1_id INTEGER, CONSTRAINT table2_table1_id_fkey FOREIGN KEY (table1_id) REFERENCES test_foreignkey.table1(id))", 
            &[],
        ).unwrap();
        let res = introspect_foreign_keys(&mut tx, &vec!["test_foreignkey"]).unwrap();
        assert_eq!(
            res,
            vec![crate::introspection::foreignkey::ForeignKey {
                constraint_name: "table2_table1_id_fkey".to_string(),
                constraint_schema: "test_foreignkey".to_string(),
                constraint_table_name: "table2".to_string(),
                columns: vec!["table1_id".to_string()],
                target_table_name: "table1".to_string(),
                target_table_schema: "test_foreignkey".to_string(),
                target_table_columns: vec!["id".to_string()],
                deferrable: false,
                initially_deferred: false,
                match_option: "NONE".to_string(),
                update_rule: "NO ACTION".to_string(),
                delete_rule: "NO ACTION".to_string(),
            }],
        );
    }
}
