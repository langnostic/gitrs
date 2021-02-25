use std::io;
use std::path::PathBuf;

pub fn run_init(cmd_dir: Option<&str>) -> io::Result<()> {
    let git_path = create_path(cmd_dir)?;
    // check if folder already exists
    if git_path.exists() {}
    //git_path.push(".gitrs/");
    Ok(())
}

fn create_path(cmd_path: Option<&str>) -> io::Result<PathBuf> {
    Ok(match sanitize_path_str(cmd_path) {
        Some(s) => PathBuf::from(s),
        None => std::env::current_dir()?,
    })
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

// #[cfg(test)]
// mod create_path_tests {
//     use super::*;

//     fn setup_tmp_dir(path: Option<&str>) -> io::Result<PathBuf> {
//         let tmp_path = PathBuf::from(path.unwrap_or("/tmp"));
//         if !tmp_path.exists() {
//             std::fs::create_dir(&tmp_path)?;
//         }
//         std::env::set_current_dir(&tmp_path)?;
//         Ok(tmp_path)
//     }

//     #[test]
//     fn create_path_with_none() -> io::Result<()> {
//         let tmp_path = setup_tmp_dir(None)?;
//         let actual_path = create_path(None)?;
//         assert_eq!(tmp_path, actual_path);
//         Ok(())
//     }

//     #[test]
//     fn create_path_with_multi_slashes() -> io::Result<()> {
//         let tmp_path = setup_tmp_dir(None)?;
//         let actual_path = create_path(Some("/tmp/////"))?;
//         assert_eq!(tmp_path, actual_path);
//         Ok(())
//     }
// }
