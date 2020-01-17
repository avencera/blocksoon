use clap::ArgMatches;

#[derive(Debug)]
pub struct Options {
    pub countdown: usize,
}

impl Options {
    pub fn new_from_matches(matches: &ArgMatches) -> Options {
        let (hours, minutes, seconds) = time_selector_tuple(matches);
        let total_seconds = (hours * 60 * 60) + (minutes * 60) + seconds;

        Options {
            countdown: total_seconds,
        }
    }
}

fn time_selector_tuple(matches: &ArgMatches) -> (usize, usize, usize) {
    (
        matches
            .value_of("hours")
            .map(|v| v.parse().expect("Unable to parse"))
            .unwrap_or(0),
        matches
            .value_of("minutes")
            .map(|v| v.parse().expect("Unable to parse"))
            .unwrap_or(0),
        matches
            .value_of("seconds")
            .map(|v| v.parse().expect("Unable to parse"))
            .unwrap_or(0),
    )
}
