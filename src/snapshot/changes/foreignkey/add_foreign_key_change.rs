use super::super::Change;
use super::DropForeignKeyChange;
use crate::snapshot::{Database, ForeignKey, SnapshotError};
use crate::util::sqlfmt::{sql_qa, sql_ql, sql_qn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddForeignKeyChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
    pub columns: Vec<String>,
    pub target_schema: String,
    pub target_table: String,
    pub target_columns: Vec<String>,
    pub match_option: String,
    pub update_rule: String,
    pub delete_rule: String,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

impl AddForeignKeyChange {
    pub fn new(t: &ForeignKey) -> Self {
        Self {
            schema: t.schema_name.clone(),
            table: t.table_name.clone(),
            constraint: t.name.clone(),
            columns: t.columns.clone(),
            target_schema: t.target_schema.clone(),
            target_table: t.target_table.clone(),
            target_columns: t.target_columns.clone(),
            match_option: t.match_option.clone(),
            update_rule: t.update_rule.clone(),
            delete_rule: t.delete_rule.clone(),
            deferrable: t.deferrable,
            initially_deferred: t.initially_deferred,
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;

        let foreign_key = ForeignKey {
            schema_name: self.schema.clone(),
            table_name: self.table.clone(),
            name: self.constraint.clone(),
            columns: self.columns.clone(),
            target_schema: self.target_schema.clone(),
            target_table: self.target_table.clone(),
            target_columns: self.target_columns.clone(),
            match_option: self.match_option.clone(),
            update_rule: self.update_rule.clone(),
            delete_rule: self.delete_rule.clone(),
            deferrable: self.deferrable,
            initially_deferred: self.initially_deferred,
        };

        table.add_constraint(foreign_key.into())?;

        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "ALTER TABLE {} ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({}) {} {} {}",
            sql_qa(&self.schema, &self.table),
            sql_qn(&self.constraint),
            sql_ql(&self.columns),
            sql_qa(&self.target_schema, &self.target_table),
            sql_ql(&self.target_columns),
            self.match_option,
            self.update_rule,
            self.delete_rule,
        ) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;

        Ok(DropForeignKeyChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: self.constraint.clone(),
        }
        .into())
    }
}
