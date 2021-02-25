extern crate clap;
mod init;
mod repository;

use clap::{crate_version, App, AppSettings, Arg};
use repository::Repository;

// Lets start by recreating the 'init' command
// I need to create a '.git' folder at the given directory
// what should happen if .git already exists?
fn main() {
    let matches = App::new("gitrs - git written in rust")
        .version(crate_version!())
        .arg(Arg::with_name("init"))
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .get_matches();

    let _main_result: Result<(), ()>;

    if matches.is_present("init") {
        _main_result = init::run_init();
    }

    let _ = Repository {};
}
