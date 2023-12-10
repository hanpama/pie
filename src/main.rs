mod cmd;
mod definition;
mod error;
mod history;
mod introspection;
mod project;
mod snapshot;
mod util;

use clap::{ArgMatches, Command};

fn main() {
    let cli = Command::new("pie")
        .bin_name("pie")
        .version("0.1.0")
        .author("Kyungil Choi <hanpama@gmail.com>")
        .about("PostgreSQL schema management tool")
        .subcommand_required(true)
        .subcommand(
          Command::new("init")
          .about("initializes base project structure")
        )
        .subcommand(
          Command::new("status")
          .about("show the workspace and database version status")
          .arg(clap::arg!(--"profile" <String>).default_value("database"))
        )
        .subcommand(
          Command::new("stage")
          .about("calculates diff and puts it to the stage area")
          .arg(clap::arg!(--"profile" <String>).default_value("database"))
        )
        .subcommand(
          Command::new("reset")
          .about("resets the stage area")
          .arg(clap::arg!(--"profile" <String>).default_value("database"))
        )
        .subcommand(
          Command::new("make")
          .about("fleushes the staged diff and makes it as a version")
          .arg(clap::arg!(--"profile" <String>).default_value("database"))
          .arg(clap::arg!(--"version" <String>).default_value("auto"))
        )
        .subcommand(
          Command::new("print")
          .about("prints the definition snapshot")
        )
        .subcommand(
          Command::new("up")
          .about("reads the history directory and applies upward migration")
        )
        .subcommand(
          Command::new("down")
          .about("reads the history directory and applies downward migration")
        )
        .subcommand(
          Command::new("compare")
          .about("calculates the changes between the definition snapshot and the database snapshot. when --apply flag is specified, it applies the changes to the database.")
        )
        .subcommand(
          Command::new("clone")
          .about("inspects the database and creates snapshot and the initial migration")
        );

    let matches = cli.get_matches();

    // let dotpieyaml = std::path::Path::new(".pie.yaml");
    // // resolve the workspace root

    let res = match matches.subcommand() {
        Some(("init", _)) => cmd::init(),
        Some(("status", args)) => cmd::status(get_profile(args)),
        Some(("stage", args)) => cmd::stage(get_profile(args)),
        Some(("reset", args)) => cmd::reset(get_profile(args)),
        Some(("make", args)) => cmd::make(get_profile(args), get_version(args)),
        // Some(("print", _)) => cmd::print(),
        // Some(("up", _)) => cmd::up(),
        // Some(("down", _)) => cmd::down(),
        // Some(("compare", _)) => cmd::compare(),
        // Some(("clone", _)) => cmd::clone(),
        _ => unreachable!(),
    };

    res.unwrap();
}

fn get_profile(args: &ArgMatches) -> &String {
    args.get_one::<String>("profile").unwrap()
}
fn get_version(args: &ArgMatches) -> &String {
    args.get_one::<String>("version").unwrap()
}
