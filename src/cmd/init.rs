use crate::definition;
use crate::error::AnyError;
use crate::history::{FSHistory, Version};
use crate::project::{discover_project, initialize_project};
use crate::snapshot::Database;

pub fn init() -> Result<(), AnyError> {
    let cwd = std::env::current_dir()?;

    if let Ok(proj) = discover_project(cwd.clone()) {
        println!("Project already exists at: {}", proj.base_dir.display());
        return Ok(());
    }

    let project = initialize_project(cwd)?;
    let history_dir = project.resolve_default_history_dir()?;
    let definition_dir = project.resolve_default_definitions_dir()?;

    std::fs::create_dir_all(&history_dir)?;
    std::fs::create_dir_all(&definition_dir)?;

    let init = Version::new_init();
    let stage = Version::new_stage(&init.name);

    let mut snapshot = Database::new();

    for change in init.changes.iter() {
        change.apply(&mut snapshot)?;
    }
    for change in stage.changes.iter() {
        change.apply(&mut snapshot)?;
    }

    let mut fsh = FSHistory::new(history_dir.clone());
    fsh.save_version(init)?;
    fsh.save_version(stage)?;

    definition::save_snapshot(&definition_dir.join("public.yaml"), &snapshot)?;

    Ok(())
}
