use crate::{
    error::AnyError,
    history::{FSHistory, Version},
    project::discover_project,
};
use colored::Colorize;

pub fn make(profile_name: &str, version: &str) -> Result<(), AnyError> {
    let cwd = std::env::current_dir()?;
    let project = discover_project(cwd.clone())?;
    let history_dir = project.resolve_history_dir(profile_name)?;

    let mut fsh = FSHistory::from_dir(&history_dir)?;

    let mut to_make = fsh.get_version("stage")?;

    if to_make.is_empty() {
        println!("Nothing to make. Stage is empty.");
        return Ok(());
    }
    to_make.name = format!(
        "{}-{}",
        chrono::Local::now().format("%Y%m%d_%H%M%S"),
        version
    );
    let new_stage = Version::new_stage(&to_make.name);

    println!("Made version: {}", &to_make.name.green());

    fsh.save_version(to_make)?;
    fsh.save_version(new_stage)?;

    Ok(())
}
