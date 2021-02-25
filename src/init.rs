use std::path::PathBuf;

pub fn run_init(cmd_dir: Option<&str>) -> Result<(), std::io::Error> {
    let git_path = create_path(cmd_dir)?;
    // check if folder already exists
    if git_path.exists() {}
    //git_path.push(".gitrs/");
    Ok(())
}

fn create_path(cmd_path: Option<&str>) -> Result<PathBuf, std::io::Error> {
    Ok(match cmd_path {
        Some(s) => PathBuf::from(sanitize_path_str(s)),
        None => std::env::current_dir()?,
    })
}

// strip any slashes from user input str
// PathBuf will add slashes as needed
// FIXME: does this 'path delimiter' need to be cross platform?
fn sanitize_path_str(path_str: &str) -> &str {
    let mut s = path_str.trim();
    while s.ends_with('/') {
        s = s.strip_suffix('/').unwrap();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_tmp_dir(path: Option<&str>) -> std::io::Result<PathBuf> {
        let tmp_path = PathBuf::from(path.unwrap_or("/tmp"));
        if !tmp_path.exists() {
            std::fs::create_dir(&tmp_path)?;
        }
        std::env::set_current_dir(&tmp_path)?;
        Ok(tmp_path)
    }

    #[test]
    fn sanitize_path_str_one_slash() {
        // arrange
        let path_str = "/tmp/";

        // act
        let actual_path = sanitize_path_str(path_str);

        // assert
        assert_eq!("/tmp", actual_path);
    }

    #[test]
    fn sanitize_path_str_no_slash() {
        let path_str = "/tmp";
        let actual = sanitize_path_str(path_str);
        assert_eq!(path_str, actual);
    }

    #[test]
    fn create_path_with_none() -> std::io::Result<()> {
        let tmp_path = setup_tmp_dir(None)?;

        let actual_path = create_path(None)?;

        assert_eq!(tmp_path, actual_path);
        Ok(())
    }

    #[test]
    fn create_path_with_multi_slashes() -> std::io::Result<()> {
        let tmp_path = setup_tmp_dir(None)?;

        let actual_path = create_path(Some("/tmp/////"))?;

        assert_eq!(tmp_path, actual_path);
        Ok(())
    }
}
