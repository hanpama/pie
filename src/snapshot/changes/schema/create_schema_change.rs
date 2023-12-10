use super::DropSchemaChange;
use crate::snapshot::{changes::Change, Database, Schema, SnapshotError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateSchemaChange {
    pub schema_name: String,
}

impl CreateSchemaChange {
    pub fn new(t: &Schema) -> Self {
        Self {
            schema_name: t.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        source.add_schema(Schema::new(&self.schema_name))?;
        return Ok(());
    }

    pub fn render_sql(&self) -> String {
        format!("CREATE SCHEMA {};", self.schema_name)
    }

    pub fn revert(&self, source: &Database) -> Result<Change, SnapshotError> {
        Ok(DropSchemaChange {
            schema_name: self.schema_name.clone(),
        }
        .into())
    }
}
