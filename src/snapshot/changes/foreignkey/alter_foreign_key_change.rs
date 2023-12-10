use super::super::Change;
use crate::snapshot::{Database, ForeignKey, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AlterForeignKeyChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

impl AlterForeignKeyChange {
    pub fn new(t: &ForeignKey) -> Self {
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
        let foreign_key = table
            .get_constraint_mut(&self.constraint)?
            .as_foreign_key_mut()?;

        foreign_key.deferrable = self.deferrable;
        foreign_key.initially_deferred = self.initially_deferred;

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
        let foreign_key = table.get_constraint(&self.constraint)?.as_foreign_key()?;

        Ok(AlterForeignKeyChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: foreign_key.name.clone(),
            deferrable: foreign_key.deferrable,
            initially_deferred: foreign_key.initially_deferred,
        }
        .into())
    }
}
