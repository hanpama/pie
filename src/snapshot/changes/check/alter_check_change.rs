use super::super::Change;
use crate::snapshot::{Check, Database, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AlterCheckChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

impl AlterCheckChange {
    pub fn new(t: &Check) -> Self {
        Self {
            schema: t.schema_name.clone(),
            table: t.table_name.clone(),
            constraint: t.name.clone(),
            deferrable: t.deferrable,
            initially_deferred: t.initially_deferred,
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;
        let check = table.get_constraint_mut(&self.constraint)?.as_check_mut()?;

        check.deferrable = self.deferrable;
        check.initially_deferred = self.initially_deferred;

        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "ALTER TABLE {} ALTER CONSTRAINT {} {}",
            sql_qa(&self.schema, &self.table),
            sql_qn(&self.constraint),
            if self.deferrable {
                "DEFERRABLE INITIALLY DEFERRED"
            } else {
                "NOT DEFERRABLE INITIALLY IMMEDIATE"
            },
        )
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;
        let check = table.get_constraint(&self.constraint)?.as_check()?;

        Ok(AlterCheckChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: check.name.clone(),
            deferrable: check.deferrable,
            initially_deferred: check.initially_deferred,
        }
        .into())
    }
}
