use std::fs;
use std::io;
use std::path::PathBuf;

pub fn init_main(cmd_dir: Option<&str>) -> io::Result<()> {
    let git_path = create_git_path(cmd_dir)?;

    if !git_path.exists() {
        fs::create_dir(&git_path)?;
    }

    Ok(create_git_tree(&git_path)?)
}

fn create_git_tree(git_path: &PathBuf) -> io::Result<()> {
    // create git directories
    for dir in [
        "objects",
        "objects/info",
        "objects/pack",
        "refs",
        "refs/heads",
        "refs/tags",
    ]
    .iter()
    {
        fs::create_dir(git_path.join(dir))?;
    }

    // create git HEAD file
    fs::write(git_path.join("HEADS"), "refs: refs/heads/main\n")?;

    Ok(())
}

fn create_git_path(cmd_path: Option<&str>) -> io::Result<PathBuf> {
    let dir = match sanitize_path_str(cmd_path) {
        Some(s) => PathBuf::from(s),
        None => std::env::current_dir()?,
    };
    Ok(dir.join(".gitrs"))
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
        let tmp_path = setup_tmp_dir(None)?.join(".gitrs");
        let actual_path = create_git_path(Some("/tmp/////"))?;
        assert_eq!(tmp_path, actual_path);
        Ok(())
    }
}
