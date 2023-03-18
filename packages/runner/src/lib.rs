use std::path::Path;

use log::info;
use subprocess::Exec;

pub fn execute(cmdstr: &String, cwd: &Path) {
    let result = Exec::shell(cmdstr)
        .cwd(cwd)
        .capture()
        .expect("Sub process failed");
    info!("{:?}", result.exit_status);
    info!("{:?}", result.stdout_str());
}
