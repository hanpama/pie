use super::super::Change;
use crate::snapshot::{Column, Database, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AlterColumnSetNotNullChange {
    pub schema: String,
    pub table: String,
    pub column: String,
    pub not_null: bool,
}

impl AlterColumnSetNotNullChange {
    pub fn new(t: &Column) -> Self {
        Self {
            schema: t.schema_name.clone(),
            table: t.table_name.clone(),
            column: t.name.clone(),
            not_null: t.not_null,
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;
        let column = table.get_column_mut(&self.column)?;

        column.not_null = self.not_null;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        if self.not_null {
            format!(
                "ALTER TABLE {} ALTER COLUMN {} SET NOT NULL;",
                sql_qa(&self.schema, &self.table),
                sql_qn(&self.column),
            )
        } else {
            format!(
                "ALTER TABLE {} ALTER COLUMN {} DROP NOT NULL;",
                sql_qa(&self.schema, &self.table),
                sql_qn(&self.column),
            )
        }
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;
        let column = table.get_column(&self.column)?;

        Ok(AlterColumnSetNotNullChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            column: column.name.clone(),
            not_null: column.not_null,
        }
        .into())
    }
}
