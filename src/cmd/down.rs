use crate::{error::AnyError, history::DBHistory, project::discover_project};
use colored::Colorize;
use postgres::{Client, NoTls};

pub fn down(profile_name: &str, version: Option<&str>) -> Result<(), AnyError> {
    let cwd = std::env::current_dir()?;
    let project = discover_project(cwd.clone())?;

    let database_url = project.resolve_database_url(profile_name)?;
    let metadata_schema = project.resolve_metadata_schema(&profile_name)?;

    let mut pg_client = Client::connect(&database_url, NoTls)?;
    let mut tx = pg_client.transaction()?;

    let mut dbh = DBHistory::new(&metadata_schema);

    dbh.ensure_initialized(&mut tx)?;

    let current_version = dbh.get_current_version(&mut tx)?;

    println!("On database version: {}", current_version.name);
    if version.is_none() && current_version.previous.is_none() {
        println!("No migrations to apply. Already on the init version.");
        return Ok(());
    }
    let from_version = current_version.name.clone();
    let to_version = match version {
        Some(v) => v.to_owned(),
        None => current_version.previous.unwrap(),
    };
    println!("Applying downward migrations to {}:", to_version);
    let versions = dbh.get_downward_range(&mut tx, &from_version, &to_version)?;
    
    if versions.len() == 0 {
        println!("  No migrations to apply.");
        return Ok(());
    }

    for name in versions {
        let v = dbh.get_version(&mut tx, &name).unwrap();
        println!("  Reverting version {}", v.name.green());
        for stmt in &v.down {
            tx.execute(stmt, &[]).unwrap();
            println!("    {}", stmt.dimmed());
        }
        dbh.delete_version(&mut tx, &v).unwrap();
    }

    tx.commit()?;

    Ok(())
}
