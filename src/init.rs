use crate::repo::Repository;

pub fn main(args: &[String]) {
    let dir = match args.len() {
        0 => ".",
        _ => args[0].as_str(),
    };

    let repo = Repository::new(dir);

    println!(
        "Initialized empty Gitrs repository in {}",
        repo.path.display()
    );
}
