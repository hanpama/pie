use super::{super::Change, DropFunctionChange};
use crate::snapshot::{Database, Function, SnapshotError};
use crate::util::sqlfmt::sql_qa;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFunctionChange {
    pub schema: String,
    pub function: String,
    pub body: String,
    pub language: String,
    pub returns: String,
    pub volatility: String,
}

impl CreateFunctionChange {
    pub fn new(t: &Function) -> Self {
        Self {
            schema: t.schema_name.clone(),
            function: t.name.clone(),
            body: t.body.clone(),
            language: t.language.clone(),
            returns: t.returns.clone(),
            volatility: t.volatility.clone(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;

        schema.add_function(Function {
            schema_name: self.schema.clone(),
            name: self.function.clone(),
            body: self.body.clone(),
            language: self.language.clone(),
            returns: self.returns.clone(),
            volatility: self.volatility.clone(),
        })?;
        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "CREATE FUNCTION {} {} RETURNS {} LANGUAGE {} {}",
            sql_qa(&self.schema, &self.function),
            self.body,
            self.returns,
            self.language,
            self.volatility,
        ) + ";"
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;

        Ok(DropFunctionChange {
            schema: schema.name.clone(),
            function: self.function.clone(),
        }
        .into())
    }
}
