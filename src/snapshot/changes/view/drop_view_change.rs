use super::CreateViewChange;
use crate::{
    snapshot::{changes::Change, Database, SnapshotError, View},
    util::sqlfmt,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DropViewChange {
    pub schema: String,
    pub view: String,
}

impl DropViewChange {
    pub fn new(s: &View) -> Self {
        Self {
            schema: s.schema_name.clone(),
            view: s.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        schema.get_relation(&self.view)?.as_view()?;
        schema.remove_relation(&self.view)?;
        return Ok(());
    }

    pub fn render_sql(&self) -> String {
        format!("DROP VIEW {}", sqlfmt::sql_qa(&self.schema, &self.view))
    }

    pub fn revert(&self, source: &Database) -> Result<Change, SnapshotError> {
        let schema = source.get_schema(&self.schema)?;
        let view = schema.get_relation(&self.view)?.as_view()?;

        Ok(CreateViewChange {
            schema: self.schema.clone(),
            view: self.view.clone(),
            query: view.query.clone(),
        }
        .into())
    }
}
