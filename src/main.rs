mod cmd;
mod definition;
mod error;
mod history;
mod introspection;
mod project;
mod snapshot;
mod util;

use clap::{ArgMatches, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cli = Command::new("podo")
        .bin_name("podo")
        .version(VERSION)
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
          .arg(clap::arg!(--"version" <String>))
        )
        .subcommand(
          Command::new("up")
          .about("reads the history directory and applies upward migration")
          .arg(clap::arg!(--"profile" <String>).default_value("database"))
          .arg(clap::arg!(--"version" <String>))
        )
        .subcommand(
          Command::new("down")
          .about("reads the history directory and applies downward migration")
          .arg(clap::arg!(--"profile" <String>).default_value("database"))
          .arg(clap::arg!(--"version" <String>))
        )
        .subcommand(
          Command::new("clone")
          .about("inspects the database and creates snapshot and the initial migration")
        )
        .subcommand(
          Command::new("print")
          .about("prints the definition snapshot")
        )
        .subcommand(
          Command::new("sync")
          .about("calculates the changes between the definition snapshot and the database snapshot and sets the database version it to the given version")
          .arg(clap::arg!(--"profile" <String>).default_value("database"))
        );

    let matches = cli.get_matches();

    let res = match matches.subcommand() {
        Some(("init", _)) => cmd::init(),
        Some(("status", args)) => cmd::status(get_profile(args)),
        Some(("stage", args)) => cmd::stage(get_profile(args)),
        Some(("reset", args)) => cmd::reset(get_profile(args)),
        Some(("make", args)) => cmd::make(get_profile(args), get_version(args)),
        Some(("up", args)) => cmd::up(get_profile(args), get_version(args)),
        Some(("down", args)) => cmd::down(get_profile(args), get_version(args)),
        Some(("clone", args)) => cmd::clone(get_profile(args)),
        Some(("sync", args)) => cmd::sync(get_profile(args)),
        // Some(("print", _)) => cmd::print(),
        // Some(("compare", _)) => cmd::compare(),
        _ => unreachable!(),
    };
    if let Err(e) = res {
        eprintln!("{}", e);
    }
}

fn get_profile(args: &ArgMatches) -> &String {
    args.get_one::<String>("profile").unwrap()
}
fn get_version(args: &ArgMatches) -> Option<&str> {
    args.get_one::<String>("version").map(|s| s.as_str())
}
