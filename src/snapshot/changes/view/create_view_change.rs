use super::DropViewChange;
use crate::snapshot::{changes::Change, Database, SnapshotError, View};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateViewChange {
    pub schema: String,
    pub view: String,
    pub query: String,
}

impl CreateViewChange {
    pub fn new(t: &View) -> Self {
        Self {
            schema: t.schema_name.clone(),
            view: t.name.clone(),
            query: t.query.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;
        let view = View {
            schema_name: self.schema.clone(),
            name: self.view.clone(),
            query: self.query.clone(),
        };
        schema.add_relation(view.into())?;
        return Ok(());
    }

    pub fn render_sql(&self) -> String {
        format!(
            "CREATE VIEW {} AS {}",
            crate::util::sqlfmt::sql_qa(&self.schema, &self.view),
            self.query
        )
    }

    pub fn revert(&self, source: &Database) -> Result<Change, SnapshotError> {
        let schema = source.get_schema(&self.schema)?;

        Ok(DropViewChange {
            schema: schema.name.clone(),
            view: self.view.clone(),
        }
        .into())
    }
}
