use super::CreateSchemaChange;
use crate::snapshot::{changes::Change, Database, Schema, SnapshotError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropSchemaChange {
    pub schema_name: String,
}

impl DropSchemaChange {
    pub fn new(s: &Schema) -> Self {
        Self {
            schema_name: s.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        source.remove_schema(&self.schema_name)?;
        return Ok(());
    }

    pub fn render_sql(&self) -> String {
        format!("DROP SCHEMA {};", self.schema_name)
    }

    pub fn revert(&self, source: &Database) -> Result<Change, SnapshotError> {
        Ok(CreateSchemaChange {
            schema_name: self.schema_name.clone(),
        }
        .into())
    }
}
