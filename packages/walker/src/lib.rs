mod loader;
mod processor;

use std::path::{Path, PathBuf};

use glob::glob;

pub fn walk() -> Vec<processor::StepsCollection> {
    let mut hooks_files: Vec<PathBuf> = Vec::new();
    let mut steps_collections: Vec<processor::StepsCollection> = Vec::new();

    let repository_root = match git_client::get_repository_path() {
        Ok(path) => path,
        Err(error) => {
            log::error!("{}", error);
            panic!("Failed to get repository path.");
        }
    };

    // Retrieve the list of changed files
    let changed_files = match git_client::get_changed_files() {
        Ok(files) => files
            .iter()
            .map(|p| Path::new(&repository_root).join(p))
            .collect(),
        Err(error) => {
            log::error!("Encountered error when loading changed files from git.");
            log::error!("{}", error);
            log::warn!("Returning an empty list of changed files");
            Vec::<PathBuf>::new()
        }
    };

    // Build the glob pattern to match the pre-commit.json files
    let match_pattern = repository_root + "/**/.hooks/pre-commit.json";

    // Retrieve the list of pre-commit.json files, but only the ones that have changed files
    // This code is ugly, but since `glob` returns an iterator, I don't know how to collect it properly into a vector where I can iterate over it and perform filtering
    match glob(&match_pattern) {
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(p) => {
                        // p is the path to the pre-commit.json file, looking like `path/to/repo/.hooks/pre-commit.json`. We want to retrieve `path/to/repo`
                        let workspace_path = p.parent().unwrap().parent().unwrap();
                        if changed_files.iter().any(|f| f.starts_with(workspace_path)) {
                            log::info!("Collecting hook file: {:?}", p);
                            hooks_files.push(p);
                        }
                    }
                    Err(error) => {
                        log::error!("{}", error);
                    }
                }
            }
        }
        Err(error) => {
            log::error!("{}", error);
            panic!("Failed to read hook files.");
        }
    };

    // For each pre-commit file, look for the first changed files that includes the path of the pre-commit file

    for file in changed_files {
        log::debug!("Changed file: {:?}", file);
    }

    for path in hooks_files {
        let raw_collection = loader::read_from_file(path).unwrap();
        steps_collections.push(processor::process_from_raw_collection(&raw_collection));
    }

    steps_collections
}
