use postgres::Transaction;

use crate::snapshot::changes::Change;

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
    ) -> Result<(), DBHistoryError> {
        let change_json = &serde_json::to_string(&version.changes)?;

        tx.execute(
            &format_insert_version_sql(&self.metadata_schema),
            &[
                &version.name,
                &version.previous,
                &change_json,
                &version.up,
                &version.down,
            ],
        )?;
        Ok(())
    }

    pub fn delete_version(
        &mut self,
        tx: &mut Transaction,
        version: &Version,
    ) -> Result<(), DBHistoryError> {
        tx.execute(
            &format_delete_version_sql(&self.metadata_schema),
            &[&version.name],
        )?;
        Ok(())
    }

    pub fn get_version(
        &mut self,
        tx: &mut Transaction,
        version_name: &str,
    ) -> Result<Version, DBHistoryError> {
        let row = tx.query_one(
            &format_get_version_sql(&self.metadata_schema),
            &[&version_name],
        )?;
        let version = DBHistory::populate_row(row)?;
        Ok(version)
    }

    pub fn get_current_version(&mut self, tx: &mut Transaction) -> Result<Version, DBHistoryError> {
        if !self.meta_schema_exists(tx)? {
            return Err(DBHistoryError::NotInitialized);
        }
        let row = tx.query_one(&format_current_version_sql(&self.metadata_schema), &[])?;
        let version = DBHistory::populate_row(row)?;
        Ok(version)
    }

    pub fn get_downward_range(
        &mut self,
        tx: &mut Transaction,
        from: &str,
        to: &str,
    ) -> Result<Vec<String>, DBHistoryError> {
        let from_order = self.get_applied_order(tx, from)?;
        let to_order = self.get_applied_order(tx, to)?;

        if from_order.is_none() {
            return Err(DBHistoryError::NoMatchingVersion(from.to_owned()));
        }
        if to_order.is_none() {
            return Err(DBHistoryError::NoMatchingVersion(to.to_owned()));
        }

        let rows = tx.query(
            &format_get_downward_range_sql(&self.metadata_schema),
            &[&from_order, &to_order],
        )?;

        let mut versions = vec![];
        for row in rows {
            let name: String = row.get(0);
            versions.push(name);
        }

        Ok(versions)
    }

    pub fn ensure_initialized(&mut self, tx: &mut Transaction) -> Result<(), DBHistoryError> {
        if !self.meta_schema_exists(tx)? {
            self.create_meta_schema(tx)?;
        }
        Ok(())
    }

    fn create_meta_schema(&mut self, tx: &mut Transaction) -> Result<(), DBHistoryError> {
        tx.execute(&format_create_schema_sql(&self.metadata_schema), &[])?;
        tx.execute(&format_create_table_sql(&self.metadata_schema), &[])?;
        self.save_version(tx, &Version::new_init())?;
        Ok(())
    }

    fn meta_schema_exists(&mut self, tx: &mut Transaction) -> Result<bool, DBHistoryError> {
        let row = tx.query_one(
            "SELECT EXISTS(SELECT * FROM pg_catalog.pg_namespace WHERE nspname = $1)",
            &[&self.metadata_schema],
        )?;
        Ok(row.get(0))
    }

    fn get_applied_order(
        &mut self,
        tx: &mut Transaction,
        version: &str,
    ) -> Result<Option<i32>, DBHistoryError> {
        let row = tx.query_opt(
            &format_get_applied_order_sql(&self.metadata_schema),
            &[&version],
        )?;
        match row {
            Some(row) => Ok(Some(row.get(0))),
            None => Ok(None),
        }
    }

    fn populate_row(row: postgres::Row) -> Result<Version, DBHistoryError> {
        let name: String = row.get(0);
        let previous: Option<String> = row.get(1);
        let changes_json: String = row.get(2);
        let up: Vec<String> = row.get(3);
        let down: Vec<String> = row.get(4);

        let changes: Vec<Change> = serde_json::from_str(&changes_json)?;

        Ok(Version {
            name,
            previous,
            changes,
            up,
            down,
        })
    }
}

fn format_create_schema_sql(schema: &str) -> String {
    format!("CREATE SCHEMA {}", schema)
}
fn format_create_table_sql(schema: &str) -> String {
    format!(
        "CREATE TABLE {}.version (
    name VARCHAR(128) NOT NULL PRIMARY KEY,
    previous VARCHAR(128) REFERENCES {}.version(name),
    changes JSON NOT NULL,
    up TEXT[] NOT NULL,
    down TEXT[] NOT NULL,
    applied_at TIMESTAMP NOT NULL DEFAULT now(),
    applied_order SERIAL NOT NULL UNIQUE
)",
        schema, schema,
    )
}
fn format_insert_version_sql(schema: &str) -> String {
    format!(
        "INSERT INTO {}.version (name, previous, changes, up, down) VALUES ($1, $2, $3::TEXT::JSON, $4, $5)",
        schema
    )
}

fn format_current_version_sql(schema: &str) -> String {
    format!(
        "SELECT name, previous, changes::TEXT, up, down, applied_at FROM {}.version ORDER BY applied_order DESC LIMIT 1",
        schema
    )
}

fn format_delete_version_sql(schema: &str) -> String {
    format!("DELETE FROM {}.version WHERE name = $1", schema)
}

fn format_get_downward_range_sql(schema: &str) -> String {
    format!(
        "SELECT name FROM {}.version WHERE $1 >= applied_order AND applied_order > $2 ORDER BY applied_order DESC",
        schema
    )
}

fn format_get_applied_order_sql(schema: &str) -> String {
    format!(
        "SELECT applied_order FROM {}.version WHERE name = $1",
        schema
    )
}

fn format_get_version_sql(schema: &str) -> String {
    format!(
        "SELECT name, previous, changes::TEXT, up, down FROM {}.version WHERE name = $1",
        schema
    )
}

#[derive(Debug)]
pub enum DBHistoryError {
    NotInitialized,
    PostgresError(postgres::Error),
    SerdeError(serde_json::Error),
    NoMatchingVersion(String),
}

impl std::fmt::Display for DBHistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBHistoryError::NotInitialized => write!(f, "database history is not initialized"),
            DBHistoryError::PostgresError(e) => write!(f, "{}", e),
            DBHistoryError::SerdeError(e) => write!(f, "{}", e),
            DBHistoryError::NoMatchingVersion(v) => {
                write!(f, "no version matching \"{}\" found", v)
            }
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
