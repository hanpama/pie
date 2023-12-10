use serde::{Deserialize, Serialize};

use crate::{
    snapshot::{changes::Change, Column, Database, SnapshotError, Table},
    util::sqlfmt::{sql_l, sql_qa, sql_qn},
};

use super::DropTableChange;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateTableChange {
    pub schema: String,
    pub table: String,
    pub columns: Vec<CreateTableChangeColumn>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateTableChangeColumn {
    pub name: String,
    pub data_type: String,
    pub not_null: bool,
    pub default: Option<String>,
}

impl CreateTableChange {
    pub fn new(t: &Table) -> Self {
        Self {
            schema: t.schema_name.clone(),
            table: t.name.clone(),
            columns: t
                .columns
                .iter()
                .map(|c| CreateTableChangeColumn {
                    name: c.name.clone(),
                    data_type: c.data_type.clone(),
                    not_null: c.not_null,
                    default: c.default.clone(),
                })
                .collect(),
        }
    }

    pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
        let schema = source.get_schema_mut(&self.schema)?;

        let mut table = Table::new(&self.schema, &self.table);

        for column in &self.columns {
            let column = Column {
                schema_name: self.schema.clone(),
                table_name: self.table.clone(),
                name: column.name.clone(),
                data_type: column.data_type.clone(),
                not_null: column.not_null,
                default: column.default.clone(),
            };
            table.add_column(column)?;
        }

        schema.add_relation(table.into())?;

        Ok(())
    }

    pub fn render_sql(&self) -> String {
        format!(
            "CREATE TABLE {} ({});",
            sql_qa(&self.schema, &self.table),
            sql_l(self.columns.iter().map(|c| {
                let mut tokens = vec![sql_qn(&c.name), c.data_type.clone()];
                if !c.not_null {
                    tokens.push("NULL".to_string());
                }
                if let Some(default) = &c.default {
                    tokens.push(format!("DEFAULT {}", default));
                }
                tokens.join(" ")
            }))
        )
    }

    pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
        let schema = target.get_schema(&self.schema)?;

        Ok(DropTableChange {
            schema: schema.name.clone(),
            table: self.table.clone(),
        }
        .into())
    }
}
