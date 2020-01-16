use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let uid = get_uid();
    enter_sudo();

    let (send, recv) = channel();
    let coundown_duration = 2;
    let mut time = coundown_duration;

    // timer
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(coundown_duration));
        send.send(()).unwrap();
    });

    loop {
        println!("Starting self control in: {}", time);

        let timer_ended = recv.try_recv();
        if timer_ended.is_ok() {
            println!("Starting block now...");
            start_self_control(uid);
            break;
        }

        thread::sleep(Duration::from_secs(1));
        time -= 1
    }
}

fn start_self_control(uid: String) {
    Command::new("sudo")
        .arg("/Applications/SelfControl.app/Contents/MacOS/org.eyebeam.SelfControl")
        .arg(uid)
        .arg("--install")
        .output()
        .expect("Unable to start self control");
}

fn get_uid() -> String {
    let uid_ouput = Command::new("id")
        .arg("-u")
        .output()
        .expect("Unable to get uid");

    String::from_utf8_lossy(&uid_ouput.stdout).to_string()
}

fn enter_sudo() {
    Command::new("sudo")
        .arg("-i")
        .output()
        .expect("Unable to put user into sudo");
}
