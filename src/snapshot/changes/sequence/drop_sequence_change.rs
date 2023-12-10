use super::CreateSequenceChange;
use crate::{
    snapshot::{changes::Change, Database, Sequence, SnapshotError},
    util::sqlfmt::sql_qa,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropSequenceChange {
    pub schema: String,
    pub sequence: String,
}

impl DropSequenceChange {
    pub fn new(t: &Sequence) -> Self {
        Self {
            schema: t.schema_name.clone(),
            sequence: t.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        schema.get_relation(&self.sequence)?.as_sequence()?;
        schema.remove_relation(&self.sequence)?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!("DROP SEQUENCE {}", sql_qa(&self.schema, &self.sequence),) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let sequence = schema.get_relation(&self.sequence)?.as_sequence()?;
        Ok(CreateSequenceChange::new(sequence).into())
    }
}
