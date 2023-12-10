use super::AddUniqueChange;
use crate::{
    snapshot::{changes::Change, Database, SnapshotError, Unique},
    util::sqlfmt::{sql_qa, sql_ql, sql_qn},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropUniqueChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
}

impl DropUniqueChange {
    pub fn new(s: &Unique) -> Self {
        Self {
            schema: s.schema_name.clone(),
            table: s.table_name.clone(),
            constraint: s.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;
        table.get_constraint(&self.constraint)?.as_unique()?;
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
        let unique = table.get_constraint(&self.constraint)?.as_unique()?;

        Ok(AddUniqueChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: unique.name.clone(),
            columns: unique.columns.clone(),
            deferrable: unique.deferrable,
            initially_deferred: unique.initially_deferred,
        }
        .into())
    }
}
