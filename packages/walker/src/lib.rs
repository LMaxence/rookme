mod loader;
mod processor;

use glob::glob;

pub fn walk() -> Vec<processor::StepsCollection> {
    let mut steps_collections: Vec<processor::StepsCollection> = Vec::new();

    let mut match_pattern = match git_client::get_repository_path() {
        Ok(path) => path,
        Err(error) => {
            log::error!("{}", error);
            panic!("Failed to get repository path.");
        }
    };
    match_pattern.push_str("/**/.hooks/pre-commit.json");

    let changed_files = match git_client::get_changed_files() {
        Ok(files) => files,
        Err(error) => {
            log::error!("Encountered error when loading changed files from git.");
            log::error!("{}", error);
            log::warn!("Returning an empty list of changed files");
            Vec::<String>::new()
        }
    };

    let hooks_files = match glob(&match_pattern) {
        Ok(paths) => paths,
        Err(error) => {
            log::error!("{}", error);
            panic!("Failed to read hook files.");
        }
    };

    for path in hooks_files {
        match path {
            Ok(p) => {
                if !hook_is_active(&p, &changed_files) {
                    log::info!("{:?} is not active", p);

                    continue;
                }
                log::info!("{:?} is active", p);
                let raw_collection = loader::read_from_file(p).unwrap();

                steps_collections.push(processor::process_from_raw_collection(&raw_collection));
            }
            Err(error) => {
                log::error!("{}", error);
            }
        }
    }

    steps_collections
}

fn hook_is_active(hook_path: &std::path::Path, changed_files: &Vec<String>) -> bool {
    for file in changed_files {
        let candidate = hook_path.parent().unwrap().parent().unwrap();
        log::trace!("Checking if {:?} is a parent dir of {:?}", candidate, file);
        if file.starts_with(candidate.to_str().unwrap()) {
            return true;
        }
    }
    false
}
