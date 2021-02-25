extern crate clap;

use clap::{crate_version, App, AppSettings, Arg};

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
}

mod init {
    pub fn run_init() -> Result<(), ()> {
        Ok(())
    }
}
