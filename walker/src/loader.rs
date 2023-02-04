use serde;
use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

#[derive(serde::Deserialize, Debug)]
pub struct RawStep {
    /**
     *  The named of the step. Displayed in the UI and used in it to index steps and hooks
     */
    pub name: String,
    /**
     *  The command that will be invoked in `execSync`
     */
    pub command: String,
    /**
     *  A pattern string describing which changed files will trigger this step
     */
    #[serde(rename="onlyOn")]
    pub only_on: Option<String>,
    /**
     *  Should this step be awaited before starting the next one
     */
    pub serial: Option<bool>,
    /**
     *  Does this step extend a shared step
     */
    pub from: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RawStepsCollection {
    /**
     * The list of step descriptors executed with the hook
     */
    pub steps: Vec<RawStep>,
    /**
     * A boolean denoting whether a virtualenv is started of not for this hook (eg for Python)
     */
    #[serde(rename="preCommand")]
    pub pre_command: Option<String>,
}

pub fn read_from_file(path: PathBuf) -> Result<RawStepsCollection, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let collection: RawStepsCollection = serde_json::from_reader(reader)?;
    Ok(collection)
}
