mod builtins;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("Number of args passed in: {}", args.len());

    // This solution for arg parsing works for now, but it could be a problem later.
    // May need to revisit this solution in the future
    if args.len() >= 2 {
        // TODO: Do i want to shadow the first arg. Rust documentation says that this arg may not even be populated
        // Remove the first arg (path to executable) by shadowing args vec
        let args = args[1..].to_vec();

        let _result = match args[0].as_ref() {
            "add" => feature_not_implemented(),
            "commit" => feature_not_implemented(),
            "init" => builtins::init_cmd(args),
            "pull" => feature_not_implemented(),
            "push" => feature_not_implemented(),
            "status" => builtins::status_cmd(args),
            "version" | "--version" => {
                print_version();
                Ok(())
            }
            "-h" | "--help" => {
                // git opens manpages for various subcommands.
                // this is not a concern right now, but something to keep in mind
                print_git_help();
                Ok(())
            }
            _ => {
                // TODO: Git has a 'spellcheck' feature for cmds that don't make
                // TODO: Recreate that functionality here (eventually)
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

fn feature_not_implemented() -> Result<(), i32> {
    println!("This feature has not been implemented yet. Feel free to submit a PR");
    Ok(())
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
"
    );
}
