fn main() {
    // Shit, structops automatically creates a -h option
    // I might have to bite the bullet and manually manage args
    // let args: GitArgs = GitArgs::from_args();

    let args: Vec<String> = std::env::args().collect();

    // I need to find a list of all available git commands,
    // subcommands, plumbing and porcelian.

    println!("number of args: {}", args.len());
    for arg in std::env::args() {
        println!("{:?}", arg);
    }

    // TODO: simplify this conditional. It works for prototyping
    if args.len() >= 2 && args[1] == "-h" || args[1] == "--help" {
        print_git_help();
    }
    // Print the current crate version
    // if args.version {
    //     let version = option_env!("CARGO_PKG_VERSION").unwrap();
    //     println!("gitrs version: {}", version);
    //     return;
    // }
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

mod init {
    pub fn main(args: &[String]) {
        let dir = match args.len() {
            0 => ".",
            _ => args[0].as_str(),
        };

        let repo = super::repo::Repository::new(dir);

        println!(
            "Initialized empty Gitrs repository in {}",
            repo.path.display()
        );
    }
}

mod repo {
    use path_clean::PathClean;
    use std::fs;
    use std::io;
    use std::path::PathBuf;

    const DEFAULT_GIT_DIR: &'static str = ".gitrs";
    // TODO: Question for future langnostic
    // Can I replicate this folder structure as a struct?
    const DEFAULT_GIT_DIR_TREE: &[&str] = &["objects", "refs", "refs/heads"];

    // TODO: consider moving this to a template folder?
    // const DEFAULT_LOCAL_CONFIG: &str = "\
    // [core]
    // 	repositoryformatversion = 0
    // 	filemode = true
    // 	bare = false
    // 	logallrefupdates = true
    // ";

    pub struct Repository {
        pub path: PathBuf,
    }

    impl Repository {
        pub fn new(dir: &str) -> Self {
            let path = match dir {
                "." | "./" => std::env::current_dir().unwrap(),
                _ => PathBuf::from(dir).clean(),
            }
            .join(DEFAULT_GIT_DIR);

            // TODO: what does init reinitialize actually do?
            if !path.exists() {
                Self::create_repo(&path).unwrap();
            }

            Self { path }
        }

        fn create_repo(git_path: &PathBuf) -> io::Result<()> {
            for dir in DEFAULT_GIT_DIR_TREE.iter() {
                fs::create_dir_all(git_path.join(dir))?;
            }

            fs::write(git_path.join("HEAD"), "refs: refs/heads/main\n")?;
            Ok(())
        }
    }
}
