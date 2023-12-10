use super::DropPrimaryKeyChange;
use crate::{
    snapshot::{changes::Change, Database, PrimaryKey, SnapshotError},
    util::sqlfmt::{sql_qa, sql_ql, sql_qn},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AddPrimaryKeyChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
    pub columns: Vec<String>,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

impl AddPrimaryKeyChange {
    pub fn new(t: &PrimaryKey) -> Self {
        Self {
            schema: t.schema_name.clone(),
            table: t.table_name.clone(),
            constraint: t.name.clone(),
            columns: t.columns.clone(),
            deferrable: t.deferrable,
            initially_deferred: t.initially_deferred,
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation_mut(&self.table)?.as_table_mut()?;

        table.add_constraint(
            PrimaryKey {
                schema_name: self.schema.clone(),
                table_name: self.table.clone(),
                name: self.constraint.clone(),
                columns: self.columns.clone(),
                deferrable: self.deferrable,
                initially_deferred: self.initially_deferred,
            }
            .into(),
        )?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "ALTER TABLE {} ADD CONSTRAINT {} PRIMARY KEY ({}) {} {}",
            sql_qa(&self.schema, &self.table),
            sql_qn(&self.constraint),
            sql_ql(&self.columns),
            if self.deferrable {
                "DEFERRABLE".to_string()
            } else {
                "NOT DEFERRABLE".to_string()
            },
            if self.initially_deferred {
                "INITIALLY DEFERRED".to_string()
            } else {
                "INITIALLY IMMEDIATE".to_string()
            },
        ) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;

        Ok(DropPrimaryKeyChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: self.constraint.clone(),
        }
        .into())
    }
}
