extern crate clap;

use clap::{crate_version, App, AppSettings, Arg};

fn main() {
    let _ = App::new("gitrs - git written in rust")
        .version(crate_version!())
        .arg(Arg::with_name("init"))
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .get_matches();
}
