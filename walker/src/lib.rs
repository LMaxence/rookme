mod loader;
mod processor;


use glob::glob;

pub fn walk(pattern: &str) -> Vec<processor::StepsCollection> {
    let mut steps_collections: Vec<processor::StepsCollection> = Vec::new();
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let raw_collection = loader::read_from_file(path).unwrap();
                steps_collections.push(processor::process_from_raw_collection(&raw_collection));
            }
            Err(e) => println!("{:?}", e),
        }
    }
    return steps_collections;
}
