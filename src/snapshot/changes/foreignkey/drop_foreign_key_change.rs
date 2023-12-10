use super::super::Change;
use super::AddForeignKeyChange;
use crate::snapshot::{Database, ForeignKey, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropForeignKeyChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
}

impl DropForeignKeyChange {
    pub fn new(s: &ForeignKey) -> Self {
        Self {
            schema: s.schema_name.clone(),
            table: s.table_name.clone(),
            constraint: s.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;
        table.get_constraint(&self.constraint)?.as_foreign_key()?;
        table.remove_constraint(&self.constraint)?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "ALTER TABLE {} DROP CONSTRAINT {}",
            sql_qa(&self.schema, &self.table),
            sql_qn(&self.constraint),
        )
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;
        let foreign_key = table.get_constraint(&self.constraint)?.as_foreign_key()?;

        Ok(AddForeignKeyChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: foreign_key.name.clone(),
            columns: foreign_key.columns.clone(),
            target_schema: foreign_key.target_schema.clone(),
            target_table: foreign_key.target_table.clone(),
            target_columns: foreign_key.target_columns.clone(),
            match_option: foreign_key.match_option.clone(),
            update_rule: foreign_key.update_rule.clone(),
            delete_rule: foreign_key.delete_rule.clone(),
            deferrable: foreign_key.deferrable,
            initially_deferred: foreign_key.initially_deferred,
        }
        .into())
    }
}
