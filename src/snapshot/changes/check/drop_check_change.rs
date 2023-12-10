use super::super::Change;
use super::AddCheckChange;
use crate::snapshot::{Check, Database, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropCheckChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
}

impl DropCheckChange {
    pub fn new(s: &Check) -> Self {
        Self {
            schema: s.schema_name.clone(),
            table: s.table_name.clone(),
            constraint: s.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;
        table.get_constraint(&self.constraint)?.as_check()?;
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
        let check = table.get_constraint(&self.constraint)?.as_check()?;

        Ok(AddCheckChange {
            schema: self.schema.clone(),
            table: self.table.clone(),
            constraint: self.constraint.clone(),
            expression: check.expression.clone(),
            deferrable: check.deferrable,
            initially_deferred: check.initially_deferred,
        }
        .into())
    }
}
