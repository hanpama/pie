use postgres::Transaction;

use crate::{
    error::AnyError,
    snapshot::{Database, Schema},
};

mod check;
mod column;
mod foreignkey;
mod function;
mod index;
mod primarykey;
mod schema;
mod sequence;
mod table;
mod unique;
mod view;

pub fn introspect(tx: &mut Transaction, schemas: &[&str]) -> Result<Database, AnyError> {
    let mut database = Database::new();

    load_schemas(tx, &mut database, schemas)?;
    Ok(database)
}

fn load_schemas(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for ischema in schema::introspect_schemas(tx, schemas)? {
        let schema = Schema::new(&ischema.name);
        database.add_schema(schema)?;
    }
    Ok(())
}
