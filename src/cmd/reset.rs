use crate::{
    error::AnyError,
    history::{FSHistory, STAGE},
    project::discover_project,
};

pub fn reset(profile_name: &str) -> Result<(), AnyError> {
    let cwd = std::env::current_dir()?;
    let project = discover_project(cwd.clone())?;
    let history_dir = project.resolve_history_dir(profile_name)?;
    
    let mut fsh = FSHistory::from_dir(&history_dir)?;
    let mut stage = fsh.get_version(STAGE)?;

    stage.reset();

    fsh.save_version(stage)?;

    Ok(())
}
