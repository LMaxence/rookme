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

    let _changed_files = match git_client::get_changed_files() {
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
                log::info!("{:?}", p);
                let raw_collection = loader::read_from_file(p).unwrap();

                log::info!("{:?}", raw_collection);

                steps_collections.push(processor::process_from_raw_collection(&raw_collection));
            }
            Err(error) => {
                log::error!("{}", error);
            }
        }
    }

    steps_collections
}
