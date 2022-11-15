use std::{thread, time::Duration};

fn main() {
    let v = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || {
        println!("Here is a vector {:?}", v);
    });
    handle.join().unwrap();

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
