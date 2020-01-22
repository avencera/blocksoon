pub mod options;
pub mod self_control;
use chrono::Datelike;
use notify_rust::Notification;
use std::process::Command;

pub fn enter_sudo() {
    // refreshes sudo timestamp so we can still run SelfControl in sudo even
    // after a long timer
    Command::new("sudo")
        .arg("-v")
        .output()
        .expect("Unable to put user into sudo");
}

pub fn start_self_control(uid: String) {
    let _ = Notification::new()
        .summary("BlockSoon")
        .body("Starting SelfControl block now")
        .sound_name("Glass")
        .show();

    Command::new("sudo")
        .arg("/Applications/SelfControl.app/Contents/MacOS/org.eyebeam.SelfControl")
        .arg(uid)
        .arg("--install")
        .output()
        .expect("Unable to start self control");

    // clears sudo timestamp so that next sudo command will require password
    Command::new("sudo")
        .arg("-K")
        .output()
        .expect("Unable to remove sudo privileges");
}

pub fn get_uid() -> String {
    let uid_ouput = Command::new("id")
        .arg("-u")
        .output()
        .expect("Unable to get uid");

    String::from_utf8_lossy(&uid_ouput.stdout).to_string()
}

pub fn check_if_already_running(options: &options::Options) -> () {
    // SelfControl sets started_date to a very large year when its NOT started
    // usually year 4000 or 4001
    match &options.self_control {
        Some(self_control) if self_control.block_started_date.year() >= 3000 => (),
        _ => {
            let _ = Notification::new()
                .summary("BlockSoon")
                .body("SelfControl is already running")
                .sound_name("Pop")
                .show();

            println!("SelfControl is already running");
            std::process::exit(1)
        }
    }
}
