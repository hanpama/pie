use crate::{
    snapshot::{changes::Change, Database, SnapshotError, Table},
    util::sqlfmt::sql_qa,
};
use serde::{Deserialize, Serialize};

use super::CreateTableChange;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DropTableChange {
    pub schema: String,
    pub table: String,
}

impl DropTableChange {
    pub fn new(t: &Table) -> Self {
        Self {
            schema: t.schema_name.clone(),
            table: t.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        schema.remove_relation(&self.table)?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!("DROP TABLE {}", sql_qa(&self.schema, &self.table),)
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;

        Ok(CreateTableChange::new(table).into())
    }
}
