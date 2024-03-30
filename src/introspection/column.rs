use postgres::{Error, Transaction};

#[derive(Debug, PartialEq)]
pub struct Column {
    pub schema: String,
    pub table: String,
    pub att_num: i16,
    pub name: String,
    pub default: String,
    pub not_null: bool,
    pub data_type: String,
}

pub fn introspect_columns(client: &mut Transaction, schemas: &[&str]) -> Result<Vec<Column>, Error> {
    let query = include_str!("column.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = Column {
            schema: row.get(0),
            table: row.get(1),
            att_num: row.get(2),
            name: row.get(3),
            default: row.get(4),
            not_null: row.get(5),
            data_type: row.get(6),
        };        

        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_columns;
    use crate::{introspection::column::Column, util::test::get_test_connection};

    #[test]
    fn test_introspect_columns() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_column", &[]).unwrap();
        tx.execute("
            CREATE TABLE test_column.table1 (
                col_bool bool,
                col_varchar varchar(255),
                col_numeric numeric(13, 4),
                col_array text[]
            );
        ", &[]).unwrap();
        let res = introspect_columns(&mut tx, &vec!["test_column"]).unwrap();
        assert_eq!(res, vec![
            Column {
                schema: "test_column".to_string(),
                table: "table1".to_string(),
                att_num: 1,
                name: "col_bool".to_string(),
                default: "".to_string(),
                not_null: false,
                data_type: "bool".to_string(),
            },
            Column {
                schema: "test_column".to_string(),
                table: "table1".to_string(),
                att_num: 2,
                name: "col_varchar".to_string(),
                default: "".to_string(),
                not_null: false,
                data_type: "varchar(255)".to_string(),
            },
            Column {
                schema: "test_column".to_string(),
                table: "table1".to_string(),
                att_num: 3,
                name: "col_numeric".to_string(),
                default: "".to_string(),
                not_null: false,
                data_type: "numeric(13,4)".to_string(),
            },
            Column {
                schema: "test_column".to_string(),
                table: "table1".to_string(),
                att_num: 4,
                name: "col_array".to_string(),
                default: "".to_string(),
                not_null: false,
                data_type: "text[]".to_string(),
            },
        ]);
    }
}
