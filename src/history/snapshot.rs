use crate::{error::AnyError, snapshot::Database};

use super::{FSHistory, INIT};

pub fn calculate_snapshot(history: &FSHistory, version: &str) -> Result<Database, AnyError> {
    let mut snapshot = Database::new();
    let versions = history.get_upward_range(INIT, version)?;

    for version in versions {
        let changes = version.changes;
        for change in changes {
            change.apply(&mut snapshot)?;
        }
    }
    return Ok(snapshot);
}
