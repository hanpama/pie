use super::DropIndexChange;
use crate::{
    snapshot::{changes::Change, Database, Index, SnapshotError},
    util::sqlfmt::{sql_l, sql_qa, sql_qn},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateIndexChange {
    pub schema: String,
    pub table: String,
    pub index: String,

    pub unique: bool,
    pub method: String,
    pub key_expressions: Vec<String>,
}

impl CreateIndexChange {
    pub fn new(t: &Index) -> Self {
        Self {
            schema: t.schema_name.clone(),
            table: t.table_name.clone(),
            index: t.name.clone(),
            unique: t.unique,
            method: t.method.clone(),
            key_expressions: t.key_expressions.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let table = schema.get_relation(&self.table)?.as_table()?;

        let index = Index {
            schema_name: self.schema.clone(),
            table_name: table.name.clone(),
            name: self.index.clone(),
            unique: self.unique,
            method: self.method.clone(),
            key_expressions: self.key_expressions.clone(),
        };
        schema.add_relation(index.into())?;

        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "CREATE {} {} ON {} USING {} ({})",
            if self.unique { "UNIQUE INDEX" } else { "INDEX" },
            sql_qn(&self.index),
            sql_qa(&self.schema, &self.table),
            self.method,
            sql_l(&self.key_expressions),
        ) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;

        Ok(DropIndexChange {
            schema: schema.name.clone(),
            index: self.index.clone(),
        }
        .into())
    }
}
