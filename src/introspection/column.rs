use postgres::{Client, Error};

pub struct Column {
    pub schema: String,
    pub table: String,
    pub att_num: i16,
    pub name: String,
    pub default: String,
    pub not_null: bool,
    pub data_type_name: String,
    pub is_array: bool,
    pub type_mod: Option<i32>,
}

fn introspect_columns(client: &mut Client, schemas: Vec<String>) -> Result<Vec<Column>, Error> {
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
            data_type_name: row.get(6),
            is_array: row.get(7),
            type_mod: row.get(8),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_columns;
    use crate::util::test::get_test_connection;

    #[test]
    fn test_introspect_check_constraints() {
        let mut conn = get_test_connection();
        introspect_columns(&mut conn, vec!["public".to_owned()]).unwrap();
    }
}
