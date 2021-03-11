struct Commit {
    parent: Option<Box<Commit>>,
    timestamp: std::timestamp,
}