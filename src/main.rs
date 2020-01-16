use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (send, recv) = channel();

    let coundown_duration = 10;
    let mut time = coundown_duration;

    // final
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(coundown_duration));
        send.send(()).unwrap();
    });

    loop {
        println!("Tick: {}", time);

        let timer_ended = recv.try_recv().map(|_reply| println!("Done"));

        if timer_ended.is_ok() {
            break;
        }

        thread::sleep(Duration::from_secs(1));
        time -= 1
    }
}
