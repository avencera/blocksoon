use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use users::get_current_uid;

fn main() {
    let (send, recv) = channel();

    let uid = get_current_uid();
    let _cmd = Command::new("sudo")
        .arg("-i")
        .output()
        .expect("Unable to put user into sudo");

    let coundown_duration = 2;
    let mut time = coundown_duration;

    // timer
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(coundown_duration));
        send.send(()).unwrap();
    });

    loop {
        println!("Starting self control in: {}", time);

        let timer_ended = recv
            .try_recv()
            .map(|_reply| println!("Starting block now..."));

        if timer_ended.is_ok() {
            let _ = Command::new("sudo")
                .arg("/Applications/SelfControl.app/Contents/MacOS/org.eyebeam.SelfControl")
                .arg(uid.to_string())
                .arg("--install")
                .output();

            break;
        }

        thread::sleep(Duration::from_secs(1));
        time -= 1
    }
}
