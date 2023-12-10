use super::{super::Change, CreateFunctionChange};
use crate::snapshot::{Database, Function, SnapshotError};
use crate::util::sqlfmt::sql_qa;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropFunctionChange {
    pub schema: String,
    pub function: String,
}

impl DropFunctionChange {
    pub fn new(s: &Function) -> Self {
        Self {
            schema: s.schema_name.clone(),
            function: s.name.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;

        schema.remove_function(&self.function)?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!("DROP FUNCTION {}", sql_qa(&self.schema, &self.function),) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;
        let function = schema.get_function(&self.function)?;

        Ok(CreateFunctionChange {
            schema: schema.name.clone(),
            function: self.function.clone(),
            body: function.body.clone(),
            language: function.language.clone(),
            returns: function.returns.clone(),
            volatility: function.volatility.clone(),
        }
        .into())
    }
}
