use super::CreateIndexChange;
use crate::{
    snapshot::{changes::Change, Database, Index, SnapshotError},
    util::sqlfmt::{sql_l, sql_qa, sql_qn},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropIndexChange {
    pub schema: String,
    pub index: String,
}

impl DropIndexChange {
    pub fn new(t: &Index) -> Self {
        Self {
            schema: t.schema_name.clone(),
            index: t.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        schema.get_relation(&self.index)?.as_index()?;
        schema.remove_relation(&self.index)?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!("DROP INDEX {}", sql_qa(&self.schema, &self.index),) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let index = schema.get_relation(&self.index)?.as_index()?;

        Ok(CreateIndexChange {
            schema: schema.name.clone(),
            table: index.table_name.clone(),
            index: index.name.clone(),
            unique: index.unique,
            method: index.method.clone(),
            key_expressions: index.key_expressions.clone(),
        }
        .into())
    }
}
