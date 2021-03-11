use path_clean::PathClean;

use crate::DEFAULT_GIT_DIR;
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
