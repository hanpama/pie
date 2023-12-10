use crate::{error::AnyError, snapshot::Database};

use super::{FSHistory, INIT};

pub fn calculate_snapshot(history: &FSHistory, version: &str) -> Result<Database, AnyError> {
    let mut snapshot = Database::new();
    let versions = history.get_upward_range(INIT, version)?;
    let init = history.get_version(INIT)?;

    for change in init.changes {
        change.apply(&mut snapshot)?;
    }
    for version_name in versions {
        let version = history.get_version(&version_name)?;
        let changes = version.changes;
        for change in changes {
            change.apply(&mut snapshot)?;
        }
    }
    return Ok(snapshot);
}
