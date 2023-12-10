use crate::{
    error::AnyError,
    history::{DBHistory, FSHistory},
    project::discover_project,
};

use postgres::{Client, NoTls};

pub fn up(profile_name: &str, version: Option<&str>) -> Result<(), AnyError> {
    let cwd: std::path::PathBuf = std::env::current_dir()?;
    let project = discover_project(cwd.clone())?;
    let history_dir = project.resolve_default_history_dir()?;
    let database_url = project.resolve_database_url(profile_name)?;
    let metadata_schema = project.resolve_metadata_schema(&profile_name)?;

    let mut pg_client = Client::connect(&database_url, NoTls)?;
    let mut tx = pg_client.transaction()?;

    let fsh = FSHistory::new(history_dir);
    let mut dbh = DBHistory::new(&metadata_schema);

    dbh.ensure_initialized(&mut tx)?;

    let current_version = dbh.get_current_version(&mut tx)?;

    let from_version = current_version.name.clone();
    let to_version = match version {
        Some(v) => fsh.get_version(v)?,
        None => fsh.get_current_version()?,
    };

    let versions = fsh.get_upward_range(&from_version, &to_version.name)?;

    for v in versions {
        println!("migrating to version \"{}\"", v.name);
        for stmt in &v.up {
            tx.execute(stmt, &[]).unwrap();
        }
        dbh.save_version(&mut tx, &v).unwrap();
    }

    tx.commit()?;

    Ok(())
}
