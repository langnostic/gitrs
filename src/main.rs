mod init;

static DEFAULT_GIT_DIR: &'static str = ".gitrs";
static DEFAULT_GIT_DIR_TREE: &'static [&'static str] = &[
    "objects",
    "objects/info",
    "objects/pack",
    "refs",
    "refs/heads",
    "refs/tags",
];
static DEFAULT_LOCAL_CONFIG: &'static str = "\
[core]
	repositoryformatversion = 0
	filemode = true
	bare = false
	logallrefupdates = true
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        print_short_help();
        return;
    }

    match args[1].as_str() {
        "init" => init::init_main(&args[2..]),
        "help" => print_full_help(),
        _ => {
            print_short_help();
            println!("Command not support!");
        }
    };
}

fn print_short_help() {
    println!(
        "\
    This is the help menu"
    );
}

fn print_full_help() {
    println!("");
}
