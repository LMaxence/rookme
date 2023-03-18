use std::path::Path;

use git2::{Error, Repository, Status};

static TARGET_STATUSES: &[Status] = &[
    Status::WT_NEW,
    Status::WT_MODIFIED,
    Status::WT_DELETED,
    Status::WT_RENAMED,
    Status::WT_TYPECHANGE,
];

/// Uses git2 to get the list of changed files
///
/// As git2 is a library around the libgit2 C library,
/// it can raises some errors. Like if the current working
/// directory is not a git repository, or if the git repository
/// is corrupted.
///
/// - https://libgit2.github.com/
/// - https://docs.rs/git2/latest/git2/
pub fn get_changed_files() -> Result<Vec<String>, Error> {
    let repo = get_repo()?;
    log::debug!("Successfuly using repo at path : {:?}", repo.path());
    get_files_with_status(repo)
}

pub fn get_repository_path() -> Result<String, Error> {
    let repo = get_repo()?;
    let path = repo
        .path()
        .parent()
        .expect("Could not find a parent folder for git folder.")
        .to_str()
        .expect("Failed to convert path to string");
    Ok(String::from(path))
}

// privates functions
// ------------------

/// Repository::discover will try to locate the git repository
/// starting from the working directory and going up to the root
/// If no directory is found, it will return an error.
pub fn get_repo() -> Result<Repository, Error> {
    Repository::discover(Path::new("."))
}

/// Returns the list of files that have a status in the DELTA_WHITELIST
///
/// https://docs.rs/git2/latest/git2/enum.Delta.html
fn get_files_with_status(repo: Repository) -> Result<Vec<String>, Error> {
    let statuses = repo.statuses(None).unwrap();
    let mut modified_files: Vec<String> = Vec::new();

    for status in statuses.iter() {
        let stat = status.status();
        if !TARGET_STATUSES.contains(&stat) {
            continue;
        }
        let path = status.path().unwrap();
        modified_files.push(path.to_string());
    }
    Ok(modified_files)
}
