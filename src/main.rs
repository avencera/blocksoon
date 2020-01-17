use blocksoon::options::Options;
use clap::{App, AppSettings, Arg};
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new("BlockSoon")
        .version(clap::crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .author("Praveen Perera <praveen@avencera.com>")
        .about("\nStart SelfControl after your break")
        .arg(
            Arg::with_name("hours")
                .long("hours")
                .short("h")
                .takes_value(true)
                .help("Set amount of time to countdown in hours"),
        )
        .arg(
            Arg::with_name("minutes")
                .long("minutes")
                .short("m")
                .takes_value(true)
                .help("Set amount of time to countdown in minutes"),
        )
        .arg(
            Arg::with_name("seconds")
                .long("seconds")
                .short("s")
                .takes_value(true)
                .help("Set amount of time to countdown in seconds"),
        )
        .help_short("H")
        .get_matches();

    let options = Options::new_from_matches(&matches);

    let uid = get_uid();
    enter_sudo();

    let (send, recv) = channel();
    let coundown_duration = options.seconds as u64;
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
