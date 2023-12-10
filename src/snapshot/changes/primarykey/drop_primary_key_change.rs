use super::AddPrimaryKeyChange;
use crate::{
    snapshot::{changes::Change, Database, PrimaryKey, SnapshotError},
    util::sqlfmt::{sql_qa, sql_qn},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DropPrimaryKeyChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
}

impl DropPrimaryKeyChange {
    pub fn new(s: &PrimaryKey) -> Self {
        Self {
            schema: s.schema_name.clone(),
            table: s.table_name.clone(),
            constraint: s.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;
        table.get_constraint(&self.constraint)?.as_primary_key()?;
        table.remove_constraint(&self.constraint)?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "ALTER TABLE {} DROP CONSTRAINT {}",
            sql_qa(&self.schema, &self.table),
            sql_qn(&self.constraint),
        ) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;
        let primary_key = table.get_constraint(&self.constraint)?.as_primary_key()?;

        Ok(AddPrimaryKeyChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: primary_key.name.clone(),
            columns: primary_key.columns.clone(),
            deferrable: primary_key.deferrable,
            initially_deferred: primary_key.initially_deferred,
        }
        .into())
    }
}
