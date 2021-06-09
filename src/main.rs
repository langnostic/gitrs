mod builtins;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 2 {
        // Remove the first arg (path tofilename) by shadowing vec with slice
        let args = args[1..].to_vec();

        let _result = match args[0].as_ref() {
            "init" => builtins::init_cmd(args),
            "status" => builtins::status_cmd(args),
            "version" | "--version" => {
                print_version();
                Ok(())
            }
            // TODO: maybe separate default pattern
            // and check for spelling mistakes?
            "-h" | "--help" | _ => {
                print_git_help();
                Ok(())
            }
        };

        // TODO: do something with _result
    } else {
        // No args passed in, print help screen
        print_git_help();
    }
}

fn print_version() {
    let version = option_env!("CARGO_PKG_VERSION").unwrap();
    println!("gitrs version: {:?}", version);
}

fn print_git_help() {
    // 11 chars to the first bracket '['
    // TODO: there must be a better way to print this better
    println!(
        "
usage: git [--version] [--help] [-C <path>] [-c <name>=<values>]
           [-exec-path[=<path>]] [--html-path] [--man-path] [--info-path]
           [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--bare]
           [--git-dir=<path>] [--work-tree=<path>] [--namespace=<name>]
           [--super-prefix=<path>] [--config-env=<name>=<envvar>]
           <command> [<args>]

There are the commands that are currently implemented:

start a working area (see also: git help tutorial)
   init              Create an empty Git repository or reinitialize and existing one

examine the history and state (see also: git help revision (? what the hell is that ?))
   status            Show the working tree status

\"git help -a\" and \"git help -g \" list available subcommands ane some
concept guides. See \"git help <command>\" or \"git help <concept>\"
to read about a specific subcommands or concept.
See \"git help git\" for an overview of the system"
    );
}
