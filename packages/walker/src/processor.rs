use crate::loader::RawStepsCollection;

#[derive(Debug)]
pub struct Step {
    /**
     *  The named of the step. Displayed in the UI and used in it to index steps and hooks
     */
    pub name: String,
    /**
     *  The command that will be invoked
     */
    pub command: String,
}

#[derive(Debug)]
pub struct StepsCollection {
    /**
     * The list of step descriptors executed with the hook
     */
    pub steps: Vec<Step>,
    /**
     * A boolean denoting whether a virtualenv is started of not for this hook (eg for Python)
     */
    pub pre_command: Option<String>,
    /**
     * A path from where the step is executed (cwd)
     */
    pub cwd: String,
}

pub fn process_from_raw_collection(raw_steps: &RawStepsCollection) -> StepsCollection {
    let mut steps: Vec<Step> = Vec::new();

    for raw_step in raw_steps.steps.as_slice() {
        let step = Step {
            name: raw_step.name.clone(),
            command: raw_step.command.clone(),
        };
        steps.push(step)
    }

    StepsCollection {
        steps,
        pre_command: raw_steps.pre_command.clone(),
        cwd: raw_steps.cwd.clone(),
    }
}
