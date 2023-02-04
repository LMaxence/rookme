use subprocess::Exec;
use log::info;

pub fn execute(cmdstr: &String) {
    let result = Exec::shell(cmdstr).capture().expect("Sub process failed");
    info!("{:?}", result.exit_status);
    info!("{:?}", result.stdout_str());
}
