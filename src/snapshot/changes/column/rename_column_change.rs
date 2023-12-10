use super::super::Change;
use crate::snapshot::{Column, Database, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameColumnChange {
    pub from_schema: String,
    pub from_table: String,
    pub from_column: String,
    pub to_column: String,
}

impl RenameColumnChange {
    pub fn new(s: &Column, t: &str) -> Self {
        Self {
            from_schema: s.schema_name.clone(),
            from_table: s.table_name.clone(),
            from_column: s.name.clone(),
            to_column: t.to_string(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.from_schema)?;
        let table = schema.get_relation_mut(&self.from_table)?.as_table_mut()?;
        let mut column = table.remove_column(&self.from_column)?;

        column.name = self.to_column.clone();
        table.add_column(column)?;

        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "ALTER TABLE {} RENAME COLUMN {} TO {};",
            sql_qa(&self.from_schema, &self.from_table),
            sql_qn(&self.from_column),
            sql_qn(&self.to_column),
        )
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.from_schema)?;
        let table = schema.get_relation(&self.from_table)?.as_table()?;
        let column = table.get_column(&self.to_column)?;

        Ok(RenameColumnChange {
            from_schema: schema.name.clone(),
            from_table: table.name.clone(),
            from_column: column.name.clone(),
            to_column: self.from_column.clone(),
        }
        .into())
    }
}
