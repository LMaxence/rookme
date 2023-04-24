use env_logger::{Builder, Env};
use std::path::Path;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    if !run_hooks() {
        std::process::exit(1);
    }
}

/// # Run all the hooks
///
/// Returns true if all the hooks ran successfully.
/// If one of the hooks fails, it will return false.
fn run_hooks() -> bool {
    let steps_collections = walker::walk();

    for steps_collection in steps_collections {
        log::info!("Running hooks in `{}`", steps_collection.cwd);
        for step in steps_collection.steps {
            let final_command = match steps_collection.pre_command.as_ref() {
                Some(pre) => format!("{} && {}", pre, step.command),
                None => step.command,
            };
            log::info!("\tRunning command: `{}`", final_command);
            if !runner::execute(&final_command, Path::new(&steps_collection.cwd)) {
                // here the command has failed
                // TODO: think about a proper way to make it working with parallel executions
                // early return false for the moment
                return false;
            }
        }
    }
    true
}
