use super::super::Change;
use super::AddColumnChange;
use crate::snapshot::{Column, Database, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropColumnChange {
    pub schema: String,
    pub table: String,
    pub column: String,
}

impl DropColumnChange {
    pub fn new(s: &Column) -> Self {
        Self {
            schema: s.schema_name.clone(),
            table: s.table_name.clone(),
            column: s.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;

        table.remove_column(&self.column)?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "ALTER TABLE {} DROP COLUMN {};",
            sql_qa(&self.schema, &self.table),
            sql_qn(&self.column),
        )
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;
        let column = table.get_column(&self.column)?;

        Ok(AddColumnChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            column: column.name.clone(),
            data_type: column.data_type.clone(),
            not_null: column.not_null,
            default: column.default.clone(),
        }
        .into())
    }
}
