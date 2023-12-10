use crate::{
    snapshot::{changes::Change, Database, PrimaryKey, SnapshotError},
    util::sqlfmt::{sql_qa, sql_qn},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterPrimaryKeyChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

impl AlterPrimaryKeyChange {
    pub fn new(t: &PrimaryKey) -> Self {
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
        let primary_key = table
            .get_constraint_mut(&self.constraint)?
            .as_primary_key_mut()?;

        primary_key.deferrable = self.deferrable;
        primary_key.initially_deferred = self.initially_deferred;

        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "ALTER TABLE {} ALTER CONSTRAINT {} {} {}",
            sql_qa(&self.schema, &self.table),
            sql_qn(&self.constraint),
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
        let primary_key = table.get_constraint(&self.constraint)?.as_primary_key()?;

        Ok(AlterPrimaryKeyChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: primary_key.name.clone(),
            deferrable: primary_key.deferrable,
            initially_deferred: primary_key.initially_deferred,
        }
        .into())
    }
}
