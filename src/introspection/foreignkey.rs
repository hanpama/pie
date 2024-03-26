use postgres::{Error, Transaction};

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

fn introspect_foreign_keys(
    client: &mut Transaction,
    schemas: &[&str],
) -> Result<Vec<ForeignKey>, Error> {
    let query = include_str!("foreignkey.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = ForeignKey {
            constraint_name: row.get(0),
            constraint_schema: row.get(1),
            constraint_table_name: row.get(2),
            columns: row.get(3),
            target_table_name: row.get(4),
            target_table_schema: row.get(5),
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
        introspect_foreign_keys(&mut tx, &vec!["public"]).unwrap();
    }
}
