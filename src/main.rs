use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (send, recv) = channel();

    let uid_ouput = Command::new("id")
        .arg("-u")
        .output()
        .expect("Unable to get uid");

    let uid = String::from_utf8_lossy(&uid_ouput.stdout);

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
