use crate::{
    snapshot::{changes::Change, Database, SnapshotError, Unique},
    util::sqlfmt::{sql_qa, sql_qn},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlterUniqueChange {
    pub schema: String,
    pub table: String,
    pub constraint: String,
    pub deferrable: bool,
    pub initially_deferred: bool,
}

impl AlterUniqueChange {
    pub fn new(t: &Unique) -> Self {
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
        let unique = table.get_constraint_mut(&self.constraint)?.as_unique_mut()?;

        unique.deferrable = self.deferrable;
        unique.initially_deferred = self.initially_deferred;

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
        let unique = table.get_constraint(&self.constraint)?.as_unique()?;

        Ok(AlterUniqueChange {
            schema: schema.name.clone(),
            table: table.name.clone(),
            constraint: unique.name.clone(),
            deferrable: unique.deferrable,
            initially_deferred: unique.initially_deferred,
        }
        .into())
    }
}
