use crate::{
    definition::load_snpashot,
    error::AnyError,
    history::{calculate_snapshot, DBHistory, FSHistory, Version, STAGE},
    project::discover_project,
    snapshot::compare_diff,
};
use colored::Colorize;
use postgres::{Client, NoTls};

/// The `status` command.
/// - shows the current version of the database
/// - list up the changes staged
/// - list up the changes not staged
pub fn status(profile_name: &str) -> Result<(), AnyError> {
    let cwd = std::env::current_dir()?;
    let project = discover_project(cwd.clone())?;
    let history_dir = project.resolve_default_history_dir()?;
    let definition_dir = project.resolve_default_definitions_dir()?;
    let database_url = project.resolve_database_url(profile_name)?;

    let fsh = FSHistory::new(history_dir);
    let definition_snapshot = load_snpashot(&definition_dir)?;

    let fsh_stage_version = fsh.get_version(STAGE)?;
    let fsh_stage_snapshot = calculate_snapshot(&fsh, STAGE)?;

    let metadata_schema = project.resolve_metadata_schema(&profile_name)?;
    let db_version_result = try_get_db_current_version(&database_url, &metadata_schema);

    match db_version_result {
        Ok(dbh_version) => println!("On database version: {}", dbh_version.name),
        Err(e) => println!("On database version: {}", e.to_string().red()),
    }

    if fsh_stage_version.changes.len() > 0 {
        println!("Changes staged:");
        for change in &fsh_stage_version.changes {
            println!("    {}", change.render_sql().green());
        }
    }

    let changes_not_staged = compare_diff(&fsh_stage_snapshot, &definition_snapshot);
    if changes_not_staged.len() > 0 {
        println!("Changes not staged:");
        for change in &changes_not_staged {
            println!("    {}", change.render_sql().red());
        }
    }

    if fsh_stage_version.changes.len() == 0 && changes_not_staged.len() == 0 {
        println!("No changes.");
    }

    Ok(())
}

fn try_get_db_current_version(
    database_url: &str,
    metadata_schema: &str,
) -> Result<Version, AnyError> {
    let mut pg_client = Client::connect(database_url, NoTls)?;
    let mut tx = pg_client.transaction()?;
    let mut dbh = DBHistory::new(&metadata_schema);
    let dbh_version = dbh.get_current_version(&mut tx)?;
    Ok(dbh_version)
}
