use postgres::{Error, Transaction};

pub struct View {
    pub schema: String,
    pub name: String,
    pub view_definition: String,
    pub check_option: String,
    pub is_updatable: bool,
    pub is_insertable_into: bool,
    pub is_trigger_updatable: bool,
    pub is_trigger_deletable: bool,
    pub is_trigger_insertable_into: bool,
}

pub fn introspect_views(client: &mut Transaction, schemas: &[&str]) -> Result<Vec<View>, Error> {
    let query = include_str!("view.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = View {
            schema: row.get(0),
            name: row.get(1),
            view_definition: row.get(2),
            check_option: row.get(3),
            is_updatable: row.get(4),
            is_insertable_into: row.get(5),
            is_trigger_updatable: row.get(6),
            is_trigger_deletable: row.get(7),
            is_trigger_insertable_into: row.get(8),
        };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_views;
    use crate::util::test::get_test_connection;

    #[test]
    fn test_introspect_views() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        introspect_views(&mut tx, &vec!["public"]).unwrap();
    }
}
