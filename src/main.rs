static DEFAULT_GIT_DIR: &'static str = ".gitrs";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        print_short_help();
        return;
    }

    match args[1].as_str() {
        "init" => init::main(&args[2..]),
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
    use super::DEFAULT_GIT_DIR;
    use path_clean::PathClean;
    use std::fs;
    use std::io;
    use std::path::PathBuf;

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

mod blob {}

mod commit {
    pub struct Commit {
        parent: Option<Box<Commit>>,
        // timestamp
        author: CommitAuthor,
        committer: Committer,
    }

    struct CommitAuthor {}

    struct Committer {}
}

mod tree {}
