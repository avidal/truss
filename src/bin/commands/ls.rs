use git2::{Repository, Commit, Signature};

pub fn run() {
    let repo = match Repository::open("repositories/git2-rs.git") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let head = repo.head().unwrap().target().unwrap();
    let mut commit: Commit = repo.find_commit(head).unwrap();
    {
        let summary = commit.summary().unwrap();
        println!("commit summary {}", summary);
    }
    let author: Signature = commit.author();
    println!("author {}", author);
}
