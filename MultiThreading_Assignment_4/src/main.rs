use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;

    
    // TODO: Create a channel for sending numbers
    let (tx, rx)= mpsc::channel();
    let shared_rx = Arc::new(Mutex::new(rx));
    let tx_terminate = tx.clone();
    
    
    // TODO: Create 2 producer threads
    let mut producer_handles = vec![];
    let items_per_producer = ITEM_COUNT / 2;

    for i in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(i + 1, tx_clone, items_per_producer);
        });
        producer_handles.push(handle);
    }

    drop(tx);


    
    
    // TODO: Create 3 consumer threads
    let mut consumer_handles = vec![];
    for i in 0..3 {
        let rx_clone = Arc::clone(&shared_rx);
        let handle = thread::spawn(move || {
            consumer(i + 1, rx_clone);
        });
        consumer_handles.push(handle);
    }

    
    
    // TODO: Wait for all threads to finish
    for handle in producer_handles {
        handle.join().unwrap();}

   

    for _ in 0..3 {
        tx_terminate.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let value = rng.gen_range(1..=100);
        println!("Producer {}: generated {}", id, value);
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {}: finished", id);

}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let message = {
            let rx_lock = rx.lock().unwrap();
            rx_lock.recv()
        };

        match message {
            Ok(TERMINATION_SIGNAL) => {
                println!("Consumer {}: received termination signal", id);
                break;
            }
            Ok(value) => {
                println!("Consumer {}: processing {}", id, value);
                thread::sleep(Duration::from_millis(150));
                println!("Consumer {}: done with {}", id, value);
            }
            Err(_) => {
                println!("Consumer {}: channel closed", id);
                break;
            }
        }
    }
}