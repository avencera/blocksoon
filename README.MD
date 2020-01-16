# BlockSoon

Work in progress...

Simple utility to start the [SelfControl](https://github.com/SelfControlApp/selfcontrol) app after a countdown.

## Why?

When taking a break from work, I want to give my self a break for X number of minutes, and back to it. But sometimes I'll get caught up and the break will last a lot longer than I intended. 

This solves that problem by automatically starting the SelfControl app after my break. 

## Usage (soon...)

* Start SelfControl in 10 minutes:
  * `blocksoon -m 10`

* Start SelfControl in 30 seconds:
  * `blocksoon -s 30`

* Start SelfControl in 1 hour:
  * `blocksoon -h 1`

## To Do
- [ ] Add `clap` to parse duration argument `-m` for minutes, `-s` for seconds, `-h` for hours
- [ ] Check if block is already running, if it is running don't run again
