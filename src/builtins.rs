/*
TODO: what should all the 'main' command functions return?
TODO: Probably a Result, but not sure what the variants should be.
*/

use path_clean::PathClean;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn git_status_main(args: &[String]) -> Result<(), i32> {
    // construct options for this command

    // check if first (second?) arg is '-h'
    // exit(129) ?? what is this status code ??
    for arg in args {
        println!("arg: {:?}", arg);
    }

    // TODO: need to figure out the rest of the implementation
    Ok(())
}

pub fn git_init_main(args: &[String]) -> Result<(), i32> {
    let dir = match args.len() {
        0 => ".",
        _ => args[0].as_str(),
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
