use walker;
use runner;
use log;
use env_logger::{Builder, Env};

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let steps_collections = walker::walk("**/.hooks/pre-commit.json");
    for steps_collection in steps_collections {
        log::info!("{:?}", steps_collection);
        
        let pre_command = match steps_collection.pre_command {
            Some(x) => {
                x + " && "
            },
            None => {
                "".to_string()
            }
        };

        for step in steps_collection.steps {
            let final_command = pre_command.clone() + &step.command;
            log::info!("{:?}", final_command);
            runner::execute(&final_command)
        }
    }

    
}
