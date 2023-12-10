use crate::definition::load_snpashot;
use crate::error::AnyError;
use crate::history::{calculate_snapshot, FSHistory, STAGE};
use crate::project::discover_project;
use crate::snapshot::compare_diff;
use colored::Colorize;

pub fn stage(profile_name: &str) -> Result<(), AnyError> {
    let cwd = std::env::current_dir()?;
    let project = discover_project(cwd.clone())?;
    let history_dir = project.resolve_history_dir(profile_name)?;
    let definition_dir = project.resolve_definitions_dir(profile_name)?;

    let mut fsh = FSHistory::from_dir(&history_dir)?;
    let mut target_snapshot = load_snpashot(&definition_dir)?;
    let mut source_snapshot = calculate_snapshot(&fsh, STAGE)?;

    let changes = compare_diff(&mut source_snapshot, &mut target_snapshot);
    let mut stage = fsh.get_version(STAGE)?;

    println!("Staging changes:");
    if changes.len() == 0 {
        println!("    No changes to stage.");
    }
    for change in changes {
        println!(
            "    {} {}",
            change.render_sql().green(),
            format!("-- {}", change.display_name()).dimmed(),
        );

        let up = change.render_sql();
        let down = change.revert(&source_snapshot).unwrap().render_sql();
        change.apply(&mut source_snapshot)?;

        stage.add_change(change);
        stage.add_up(&up);
        stage.add_down(&down);
    }

    fsh.save_version(stage)?;

    return Ok(());
}
