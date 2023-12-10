use std::collections::HashMap;

use crate::snapshot::error::SnapshotError;

use super::Schema;

#[derive(PartialEq, Debug)]
pub struct Database {
    pub schemas: HashMap<String, Schema>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            schemas: HashMap::new(),
        }
    }

    pub fn iter_schemas(&self) -> impl Iterator<Item = &Schema> {
        self.schemas.values()
    }
    pub fn get_schema(&self, schema: &str) -> Result<&Schema, SnapshotError> {
        self.schemas
            .get(schema)
            .ok_or(SnapshotError::schema_not_found(schema))
    }
    pub fn get_schema_mut(&mut self, schema: &str) -> Result<&mut Schema, SnapshotError> {
        self.schemas
            .get_mut(schema)
            .ok_or(SnapshotError::schema_not_found(schema))
    }
    pub fn has_schema(&self, schema: &str) -> bool {
        self.schemas.contains_key(schema)
    }
    pub fn add_schema(&mut self, schema: Schema) -> Result<(), SnapshotError> {
        if self.schemas.contains_key(&schema.name) {
            return Err(SnapshotError::schema_already_exists(&schema.name));
        }
        self.schemas.insert(schema.name.clone(), schema);
        return Ok(());
    }
    pub fn remove_schema(&mut self, schema: &str) -> Result<Schema, SnapshotError> {
        self.schemas
            .remove(schema)
            .ok_or(SnapshotError::schema_not_found(schema))
    }

    pub fn merge(&mut self, other: Database) -> Result<(), SnapshotError> {
        for (schema_name, schema) in other.schemas {
            if self.has_schema(&schema_name) {
                self.schemas.get_mut(&schema_name).unwrap().merge_schema(schema)?;
            } else {
                self.add_schema(schema)?;
            }
        }
        Ok(())
    }
}
