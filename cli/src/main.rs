use env_logger::{Builder, Env};
use std::path::Path;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let steps_collections = walker::walk();

    for steps_collection in steps_collections {
        let pre_command = match steps_collection.pre_command {
            Some(x) => x + " && ",
            None => "".to_string(),
        };

        for step in steps_collection.steps {
            let final_command = pre_command.clone() + &step.command;
            let cwd = Path::new(&steps_collection.cwd);
            log::debug!("Final command is {:?}", final_command);
            runner::execute(&final_command, cwd);
        }
    }
}
