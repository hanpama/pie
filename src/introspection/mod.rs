use postgres::Transaction;

use crate::{error::AnyError, snapshot::Database};

mod check;
mod column;

pub fn introspect(tx: Transaction) -> Result<Database, AnyError> {
    todo!()
}
