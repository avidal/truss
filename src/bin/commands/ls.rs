use std::path::Path;

use glob::glob;
use git2::{Repository, Commit, Signature};

pub fn run() {

    let entries = glob("repositories/*.git").expect("Failed to list repositories.");

    for entry in entries {
        match entry {
            Ok(path) => list_repository(path.as_path()),
            Err(e) => panic!("failed to list repository {:?}", e),
        }
    }
}

fn list_repository(path: &Path) {
    let pathstr = path.to_str().unwrap();
    println!("Path: {}", pathstr);

    let repo = match Repository::open(pathstr) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let head = repo.head().unwrap().target().unwrap();
    let mut commit: Commit = repo.find_commit(head).unwrap();
    {
        let summary = commit.summary().unwrap();
        println!("\tCommit summary {}", summary);
    }
    let author: Signature = commit.author();
    println!("\tAuthor {}", author);
}
