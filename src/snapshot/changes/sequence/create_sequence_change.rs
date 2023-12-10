use super::DropSequenceChange;
use crate::{
    snapshot::{changes::Change, Database, Sequence, SnapshotError},
    util::sqlfmt::sql_qa,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateSequenceChange {
    pub schema: String,
    pub sequence: String,

    pub data_type: String,
    pub increment: i64,
    pub min_value: i64,
    pub max_value: i64,
    pub start: i64,
    pub cache: i64,
    pub cycle: bool,
}

impl CreateSequenceChange {
    pub fn new(t: &Sequence) -> Self {
        Self {
            schema: t.schema_name.clone(),
            sequence: t.name.clone(),

            data_type: t.data_type.clone(),
            increment: t.increment,
            min_value: t.min_value,
            max_value: t.max_value,
            start: t.start,
            cache: t.cache,
            cycle: t.cycle,
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let sequence = Sequence {
            schema_name: self.schema.clone(),
            name: self.sequence.clone(),

            data_type: self.data_type.clone(),
            increment: self.increment,
            min_value: self.min_value,
            max_value: self.max_value,
            start: self.start,
            cache: self.cache,
            cycle: self.cycle,
            owned_by_table: None,
            owned_by_column: None,
        };

        schema.add_relation(sequence.into());
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "CREATE SEQUENCE {} AS {} INCREMENT BY {} MINVALUE {} MAXVALUE {} START WITH {} CACHE {} {}",
            sql_qa(&self.schema, &self.sequence),
            self.data_type,
            self.increment,
            self.min_value,
            self.max_value,
            self.start,
            self.cache,
            if self.cycle { "CYCLE" } else { "NO CYCLE" },
        ) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;

        Ok(DropSequenceChange {
            schema: schema.name.clone(),
            sequence: self.sequence.clone(),
        }
        .into())
    }
}
