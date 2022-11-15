use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side!"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("Interference"),
            String::from("in"),
            String::from("the"),
            String::from("main"),
            String::from("channel!"),
        ];

        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received from channel: {}", &received);
    }
}
