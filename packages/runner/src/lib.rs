use std::path::Path;

use log::{debug, error};
use subprocess::{Exec, ExitStatus, Redirection};

pub fn execute(cmdstr: &String, cwd: &Path) -> bool {
    debug!("Executing: `{}`", cmdstr);
    let result = Exec::shell(cmdstr)
        .cwd(cwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("Sub process failed");

    if result.exit_status != ExitStatus::Exited(0) {
        error!("Command failed: `{}`:\n{}", cmdstr, result.stdout_str());
        return false;
    }
    true
}
