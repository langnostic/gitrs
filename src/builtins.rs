/*
TODO: what should all the 'main' command functions return?
TODO: Probably a Result, but not sure what the variants should be.
*/

use path_clean::PathClean;
use std::fs;
use std::io;
use std::path::PathBuf;

fn print_args(args: &[String]) {
    for arg in args {
        println!("arg: {:?}", arg);
    }
}

pub fn status_cmd(args: Vec<String>) -> Result<(), i32> {
    print_args(&args);

    // construct options for this command

    // check if first (second?) arg is '-h'
    // exit(129) ?? what is this status code ??

    // TODO: need to figure out the rest of the implementation
    Ok(())
}

pub fn init_cmd(args: Vec<String>) -> Result<(), i32> {
    let dir = if args.len() >= 2 {
        args[1].as_str()
    } else {
        "."
    };

    let repo = Repository::new(dir);
    repo.init();

    println!(
        "Initialized empty Gitrs repository in {}",
        repo.path.display()
    );

    Ok(())
}

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
    path: PathBuf,
}

impl Repository {
    pub fn new(dir: &str) -> Self {
        let path = match dir {
            "." | "./" => std::env::current_dir().unwrap(),
            _ => PathBuf::from(dir).clean(),
        }
        .join(DEFAULT_GIT_DIR);

        Self { path }
    }

    pub fn init(&self) {
        if !self.path.exists() {
            self.create_repo().unwrap();
        } else {
            // TODO: what does reinitalization do?
            todo!()
        }
    }

    fn create_repo(&self) -> io::Result<()> {
        for dir in DEFAULT_GIT_DIR_TREE.iter() {
            fs::create_dir_all(self.path.join(dir))?;
        }

        fs::write(self.path.join("HEAD"), "refs: refs/heads/main\n")?;
        Ok(())
    }
}
