use crate::{DEFAULT_GIT_DIR, DEFAULT_GIT_DIR_TREE};
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn init_main(args: &[String]) {
    let dir = match args.len() {
        0 => None,
        _ => Some(args[0].as_str()),
    };

    let git_path = match create_git_path(dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not create git directory: {}", e);
            return;
        }
    };

    if !git_path.exists() {
        if let Err(e) = fs::create_dir(&git_path) {
            eprintln!("Could not create git directory: {}", e);
            return;
        }
    }

    if let Err(e) = create_git_tree(&git_path) {
        eprintln!("Could not create git directory: {}", e);
        return;
    }

    println!("Initialized something at something");
}

fn create_git_tree(git_path: &PathBuf) -> io::Result<()> {
    // create git directories
    for dir in DEFAULT_GIT_DIR_TREE.iter() {
        fs::create_dir(git_path.join(dir))?;
    }

    // create git HEAD file
    fs::write(git_path.join("HEAD"), "refs: refs/heads/main\n")?;

    Ok(())
}

fn create_git_path(cmd_path: Option<&str>) -> io::Result<PathBuf> {
    let dir = match sanitize_path_str(cmd_path) {
        Some(s) => PathBuf::from(s),
        None => std::env::current_dir()?,
    };
    Ok(dir.join(DEFAULT_GIT_DIR))
}

// strip any trailing slashes from user input str
// PathBuf will add slashes as needed
// does this need to be cross platform?
fn sanitize_path_str(path_str: Option<&str>) -> Option<&str> {
    let s = path_str?.trim().trim_end_matches("/");
    if s.is_empty() {
        return None;
    }
    Some(s)
}

#[cfg(test)]
mod sanitize_path_str_tests {
    use super::*;

    #[test]
    fn one_slash_in_str() {
        let actual_path = sanitize_path_str(Some("/tmp/"));
        assert_eq!("/tmp", actual_path.unwrap());
    }

    #[test]
    fn no_slashes_in_str() {
        let path_str = "/tmp";
        let actual = sanitize_path_str(Some(path_str));
        assert_eq!(path_str, actual.unwrap());
    }

    #[test]
    fn none_str() {
        assert_eq!(None, sanitize_path_str(None));
    }

    #[test]
    fn only_slash_in_str() {
        assert_eq!(None, sanitize_path_str(Some("/")));
    }

    #[test]
    fn empty_string_some() {
        assert_eq!(None, sanitize_path_str(Some("")));
    }
}

#[cfg(test)]
mod create_path_tests {
    use super::*;

    fn setup_tmp_dir(path: Option<&str>) -> io::Result<PathBuf> {
        let tmp_path = PathBuf::from(path.unwrap_or("/tmp"));
        if !tmp_path.exists() {
            std::fs::create_dir(&tmp_path)?;
        }
        std::env::set_current_dir(&tmp_path)?;
        Ok(tmp_path)
    }

    #[test]
    fn create_path_with_none() -> io::Result<()> {
        let tmp_path = setup_tmp_dir(None)?.join(".gitrs");
        let actual_path = create_git_path(None)?;
        assert_eq!(tmp_path, actual_path);
        Ok(())
    }

    #[test]
    fn create_path_with_multi_slashes() -> io::Result<()> {
        let tmp_path = setup_tmp_dir(None)?.join(DEFAULT_GIT_DIR);
        let actual_path = create_git_path(Some("/tmp/////"))?;
        assert_eq!(tmp_path, actual_path);
        Ok(())
    }
}
