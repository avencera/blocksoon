use crate::self_control::SelfControl;
use clap::ArgMatches;
use std::process;

#[derive(Debug)]
pub struct Options {
    pub countdown: usize,
    pub self_control: Option<SelfControl>,
}

impl Options {
    pub fn new_from_matches(matches: &ArgMatches) -> Options {
        let (hours, minutes, seconds) = time_selector_tuple(matches);
        let total_seconds = (hours * 60 * 60) + (minutes * 60) + seconds;

        if total_seconds == 0 {
            println!("Invalid countdown duration, please try again");
            process::exit(1);
        }

        Options {
            countdown: total_seconds,
            self_control: SelfControl::new(),
        }
    }
}

fn time_selector_tuple(matches: &ArgMatches) -> (usize, usize, usize) {
    (
        matches
            .value_of("hours")
            .map(|v| v.parse().unwrap_or(0))
            .unwrap_or(0),
        matches
            .value_of("minutes")
            .map(|v| v.parse().unwrap_or(0))
            .unwrap_or(0),
        matches
            .value_of("seconds")
            .map(|v| v.parse().unwrap_or(0))
            .unwrap_or(0),
    )
}
