use super::super::Change;
use super::DropColumnChange;
use crate::snapshot::{Column, Database, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddColumnChange {
    pub schema: String,
    pub table: String,
    pub column: String,
    pub data_type: String,
    pub not_null: bool,
    pub default: Option<String>,
}

impl AddColumnChange {
    pub fn new(t: &Column) -> Self {
        Self {
            schema: t.schema_name.clone(),
            table: t.table_name.clone(),
            column: t.name.clone(),
            data_type: t.data_type.clone(),
            not_null: t.not_null,
            default: t.default.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;

        let column = Column {
            schema_name: self.schema.clone(),
            table_name: self.table.clone(),
            name: self.column.clone(),
            data_type: self.data_type.clone(),
            not_null: self.not_null,
            default: self.default.clone(),
        };
        table.add_column(column)?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        let mut tokens = vec![format!(
            "ALTER TABLE {} ADD COLUMN {} {}",
            sql_qa(&self.schema, &self.table),
            sql_qn(&self.column),
            self.data_type,
        )];
        if !self.not_null {
            tokens.push("NOT NULL".to_string());
        }
        if let Some(default) = &self.default {
            tokens.push(format!("DEFAULT {}", default));
        }

        tokens.join(" ") + ";"
    }

    pub fn revert(&self, source: &Database) -> Result<Change, SnapshotError> {
        let schema = source.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;

        Ok(DropColumnChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            column: self.column.clone(),
        }
        .into())
    }
}
