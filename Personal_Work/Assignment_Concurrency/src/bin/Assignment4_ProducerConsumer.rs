use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Special value that signals consumers to exit
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    // Single channel shared by all producers and consumers
    let (tx, rx) = mpsc::channel::<i32>();

    // Wrap receiver in Arc<Mutex> so multiple consumer threads can share it
    let rx = Arc::new(Mutex::new(rx));

    // ── Producers ──────────────────────────────────────────────────────────
    let mut producer_handles = vec![];
    let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;

    for id in 1..=NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(id, tx_clone, items_per_producer);
        });
        producer_handles.push(handle);
    }

    // ── Consumers ──────────────────────────────────────────────────────────
    let mut consumer_handles = vec![];

    for id in 1..=NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        consumer_handles.push(handle);
    }

    // Wait for all producers to finish before sending termination signals
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Send one termination signal per consumer so every consumer exits cleanly
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer: generates random numbers and sends them to the channel
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let num: i32 = rng.gen_range(1..=100);
        println!("Producer {} sending: {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished sending.", id);
}

// Consumer: receives numbers from the shared channel and processes them
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        // Lock the receiver just long enough to pull one value
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal, exiting.", id);
            break;
        }

        println!("Consumer {} processed value: {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
}
