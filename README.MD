# BlockSoon

Simple macOS utility to start the [SelfControl](https://github.com/SelfControlApp/selfcontrol) app after a countdown.

Work in progress...

## Why?

When taking a break from work, I want to give my self a break for X number of minutes.

However, sometimes I'll get caught up and the break will last a lot longer than I intended.

This little utility solves that problem by automatically starting the SelfControl app after my break.

## Usage

Start SelfControl in 10 minutes:

- `blocksoon -m 10`

Start SelfControl in 30 seconds:

- `blocksoon -s 30`

Start SelfControl in 1 hour:

- `blocksoon -h 1`

Start SelfControl in 1 hour 10 minutes and 20 seconds:

- `blocksoon -h 1 -m10 -s 20`

## To Do

- [x] Add `clap` to parse duration argument `-m` for minutes, `-s` for seconds, `-h` for hours
- [x] Send notification when the block starts using [notify-rust](https://docs.rs/notify-rust/)
- [x] Check if block is already running, if it is running don't run again
- [ ] Reduce number of countdown logs for larger countdown times
- [ ] Add ability to run as a daemon not showing any logs
