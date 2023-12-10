use crate::{
    error::AnyError,
    history::{DBHistory, FSHistory},
    project::discover_project,
};

use postgres::{Client, NoTls};

pub fn down(profile_name: &str, version: Option<&str>) -> Result<(), AnyError> {
    let cwd = std::env::current_dir()?;
    let project = discover_project(cwd.clone())?;
    let history_dir = project.resolve_default_history_dir()?;
    let database_url = project.resolve_database_url(profile_name)?;

    let mut pg_client = Client::connect(&database_url, NoTls)?;
    let mut tx = pg_client.transaction()?;

    let fsh = FSHistory::new(history_dir);
    let mut dbh = DBHistory::new(&project.resolve_metadata_schema(&profile_name)?);

    dbh.ensure_initialized(&mut tx)?;

    let current_version = dbh.get_current_version(&mut tx)?;
    if current_version.previous.is_none() {
        println!("no need to migrate: database already has version");
        return Ok(());
    }

    let from_version = current_version.previous.clone().unwrap();
    let to_version = match version {
        Some(v) => v.to_owned(),
        None => current_version.previous.unwrap(),
    };

    let versions = fsh.get_downward_range(&from_version, &to_version)?;

    for v in versions {
        println!("migrating to version \"{}\"", v.name);
        for stmt in &v.down {
            tx.execute(stmt, &[]).unwrap();
        }
        dbh.save_version(&mut tx, &v).unwrap();
    }

    tx.commit()?;

    Ok(())
}
