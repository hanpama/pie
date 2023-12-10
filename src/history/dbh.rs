use postgres::Transaction;

use crate::{error::AnyError, snapshot::changes::Change};

use super::Version;

pub struct DBHistory {
    metadata_schema: String,
}

impl DBHistory {
    pub fn new(metadata_schema: &str) -> DBHistory {
        DBHistory {
            metadata_schema: metadata_schema.to_owned(),
        }
    }
}

impl DBHistory {
    pub fn save_version(
        &mut self,
        tx: &mut Transaction,
        version: &Version,
    ) -> Result<(), AnyError> {
        todo!()
    }

    pub fn get_current_version(&mut self, tx: &mut Transaction) -> Result<Version, DBHistoryError> {
        if !self.meta_schema_exists(tx)? {
            return Err(DBHistoryError::NotInitialized);
        }
        let row = tx.query_one(
            &format!("SELECT name, previous, changes, up, down, applied_at FROM {}.version WHERE previous IS NULL", self.metadata_schema),
            &[],
        )?;
        let name: String = row.get(0);
        let previous: Option<String> = row.get(1);
        let changes_json: String = row.get(2);
        let up_json: String = row.get(3);
        let down_json: String = row.get(4);

        let changes: Vec<Change> = serde_json::from_str(&changes_json)?;
        let up: Vec<String> = serde_json::from_str(&up_json)?;
        let down: Vec<String> = serde_json::from_str(&down_json)?;

        Ok(Version {
            name,
            previous,
            changes,
            up,
            down,
        })
    }

    pub fn ensure_initialized(&mut self, tx: &mut Transaction) -> Result<(), DBHistoryError> {
        if !self.meta_schema_exists(tx)? {
            self.create_meta_schema(tx)?;
        }
        Ok(())
    }

    fn create_meta_schema(&mut self, tx: &mut Transaction) -> Result<(), DBHistoryError> {
        tx.execute(&format!("CREATE SCHEMA {}", self.metadata_schema), &[])?;
        tx.execute(
            &format!(
                "CREATE TABLE {}.version (
            name VARCHAR(128) NOT NULL PRIMARY KEY,
            previous VARCHAR(128) NULL REFERENCES version(name),
            changes JSONB NOT NULL,
            up JSONB NULL,
            down JSONB NULL,
            applied_at TIMESTAMP NULL
        )",
                self.metadata_schema
            ),
            &[],
        )?;
        Ok(())
    }

    fn meta_schema_exists(&mut self, tx: &mut Transaction) -> Result<bool, DBHistoryError> {
        let row = tx.query_one(
            "SELECT EXISTS(SELECT * FROM pg_catalog.pg_namespace WHERE nspname = $1)",
            &[&self.metadata_schema],
        )?;
        Ok(row.get(0))
    }
}

#[derive(Debug)]
pub enum DBHistoryError {
    NotInitialized,
    PostgresError(postgres::Error),
    SerdeError(serde_json::Error),
}

impl std::fmt::Display for DBHistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBHistoryError::NotInitialized => write!(f, "database history is not initialized"),
            DBHistoryError::PostgresError(e) => write!(f, "{}", e),
            DBHistoryError::SerdeError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for DBHistoryError {}

impl From<postgres::Error> for DBHistoryError {
    fn from(e: postgres::Error) -> Self {
        DBHistoryError::PostgresError(e)
    }
}

impl From<serde_json::Error> for DBHistoryError {
    fn from(e: serde_json::Error) -> Self {
        DBHistoryError::SerdeError(e)
    }
}
