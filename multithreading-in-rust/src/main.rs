use std::sync::mpsc::{Sender, channel};
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    // Spawn a thread so we can see two threads running at the same time.
    let handlers = thread::spawn(|| {
        for i in 0..10 {
            println!("from spawned thread {}", i);
            thread::sleep(Duration::from_millis(8));
        }
    });

    // The main thread keeps working while the spawned thread is also running.
    for i in 0..10 {
        println!("from main thread {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // Wait for the first spawned thread to finish before continuing.
    handlers.join().unwrap();

    // Move ownership into a thread with `move`.
    let vec = vec![1, 2, 3];

    let handler2 = thread::spawn(move || {
        println!("vector moved into thread: {:?}", vec);
    });

    // Create a channel so multiple threads can send values back to one receiver.
    let (tx, rx) = channel();
    let tx2 = tx.clone();

    // This thread sends a few messages on the channel.
    let heandler3 = thread::spawn(move || {
        for i in 0..5 {
            let tx1 = tx.clone();
            tx1.send(format!("hello from thread {}", i)).unwrap();
        }

        // Close this sender so the receiver can eventually stop.
        drop(tx);
    });

    // Keep a list of thread handles so we can join them later.
    let mut handlers_vec: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..5 {
        let tx3 = tx2.clone();

        handlers_vec.push(thread::spawn(move || send_data(i, tx3)));
    }
    drop(tx2);

    // Read every message until all senders are dropped.
    for val in rx {
        println!("received message from worker thread: {}", val);
    }

    // Wait for the vector-moving thread to finish.
    handler2.join().unwrap();

    // Wait for every worker thread we stored in the vector.
    for handler in handlers_vec {
        handler.join().unwrap();
    }
}

fn send_data(i: i32, tx: Sender<String>) {
    tx.send(format!("message from worker thread number {}", i))
        .unwrap();
}
