use log;
use std::path::Path;

use git2::{Repository, Error, Delta};



static DELTA_WHITELIST: &'static [Delta] = &[Delta::Modified, Delta::Added];


/// Uses git2 to get the list of changed files
///
/// As git2 is a library around the libgit2 C library,
/// it can raises some errors. Like if the current working
/// directory is not a git repository, or if the git repository
/// is corrupted.
///
/// https://libgit2.github.com/
/// https://docs.rs/git2/latest/git2/
pub fn get_changed_files() -> Result<Vec<String>, Error> {
    let repo = get_repo()?;
    log::debug!("Successfuly using repo at path : {:?}", repo.path());
    let modified_files = get_files_list(repo)?; 
    Ok(modified_files)
}


// privates functions
// ------------------

/// Repository::discover will try to locate the git repository
/// starting from the working directory and going up to the root
/// If no directory is found, it will return an error.
fn get_repo() -> Result<Repository, Error> {
    Repository::discover(Path::new("."))
}

/// Returns the list of files that have a status in the DELTA_WHITELIST
///
/// https://docs.rs/git2/latest/git2/enum.Delta.html
fn get_files_list(repo: Repository) -> Result<Vec<String>, Error> {
    let statuses = repo.statuses(None).unwrap();
    let mut modified_files: Vec<String> = Vec::new();
    let mut statuses_iter = statuses.iter();
    while let Some(status) = statuses_iter.next() {
        let stat = status.index_to_workdir().unwrap().status();
        if !DELTA_WHITELIST.contains(&stat) {
            continue;
        }
        let path = status.path().unwrap();
        modified_files.push(path.to_string());
    }
    Ok(modified_files)
}
