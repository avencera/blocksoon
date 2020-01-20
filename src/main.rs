use blocksoon::options::Options;
use clap::{App, AppSettings, Arg};
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
    blocksoon::check_if_already_running(&options);

    let uid = blocksoon::get_uid();
    blocksoon::enter_sudo();

    let mut time = options.countdown as u64;

    loop {
        if time == 0 {
            println!("Starting block now...");
            blocksoon::start_self_control(uid);
            break;
        }

        // refresh sudo timestamp every 60 seconds
        if time % 60 == 0 {
            blocksoon::enter_sudo();
        }

        println!("Starting self control in: {}", time);
        thread::sleep(Duration::from_secs(1));
        time -= 1
    }
}
