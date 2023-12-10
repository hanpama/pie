use postgres::{Client, NoTls};

use crate::definition::save_snapshot;
use crate::introspection::introspect;
use crate::{error::AnyError, project::discover_project};

pub fn clone(profile_name: &str) -> Result<(), AnyError> {
    let cwd: std::path::PathBuf = std::env::current_dir()?;
    let project = discover_project(cwd.clone())?;
    let database_url = project.resolve_database_url(profile_name)?;
    let definition_dir = project.resolve_default_definitions_dir()?;

    let mut pg_client = Client::connect(&database_url, NoTls)?;
    let mut tx = pg_client.transaction()?;

    let snapshot = introspect(tx)?;

    save_snapshot(&definition_dir.join("public.yaml"), &snapshot)?;

    Ok(())
}
