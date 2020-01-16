use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (send_timer, recv_timer) = channel();
    let (send, recv) = channel();

    let coundown_duration = 10;
    let mut time = coundown_duration.clone();

    // timer
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        time = time - 1;
        send_timer.send(time).unwrap();
    });

    // final
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(coundown_duration));
        send.send(()).unwrap();
    });

    loop {
        let _ = recv_timer
            .try_recv()
            .map(|reply| println!("Tick: {}", reply));

        let timer_ended = recv.try_recv().map(|_reply| println!("Done"));

        if let Ok(_) = timer_ended {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
