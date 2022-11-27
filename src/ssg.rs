use clap::{Arg, ArgMatches, Command};
use figment::Figment;

use crate::Context;

pub(crate) fn build_command() -> Command {
    Command::new("ssg")
        .about("Building the site")
        .subcommand(
            Command::new("generate")
                .about("Generate the site")
                .arg(Arg::new("in-dir").help("The in-dir")),
        )
}

pub(crate) fn process_matches(context: &Context, _config_builder: &Figment, matches: &ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches("generate") {
        if !context.quiet {
            println!("{:#?}", _matches);
        }
    }
}

