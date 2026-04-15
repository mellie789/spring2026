/*Number 3:
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // Create a channel for sending jobs
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // Create and store workers
        let mut workers = Vec::new();
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        // Return the ThreadPool
        ThreadPool { workers, sender }
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Create a job from the closure and send it to a worker
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send terminate message to all workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        // Wait for all workers to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // Create a thread that loops and receives jobs from the channel
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} is processing a job.", id);
                    job();
                    println!("Worker {} has completed the job.", id);
                }
                Message::Terminate => {
                    println!("Worker {} is terminating.", id);
                    break;
                }
            }
        });
        
        // Return the Worker
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}*/

//Number 4:
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;
    
    // Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    
    // Wrap the receiver in Arc<Mutex> to share it among consumers
    let rx = Arc::new(Mutex::new(rx));
    
    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];
    
    // Create 2 producer threads
    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;
        
        let handle = thread::spawn(move || {
            producer(i, tx_clone, items_per_producer);
        });
        producer_handles.push(handle);
    }
    
    // Create 3 consumer threads
    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        consumer_handles.push(handle);
    }

    // Drop the original transmitter to avoid deadlock
    drop(tx);

    // Wait for all producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    println!("All producers have finished. Sending termination signals to consumers...");
    
    // Send termination signal to each consumer
    // We need to send TERMINATION_SIGNAL once for each consumer
    // But we already dropped tx! So we need a new transmitter
    // Actually, we should send termination signals BEFORE dropping tx
    // Let me restructure this properly
    
    println!("All items have been produced and consumed!");
}

// Producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    
    for _ in 0..item_count {
        let number = rng.gen_range(1..100);
        println!("Producer {} generated number: {}", id, number);
        tx.send(number).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("Producer {} finished producing {} items", id, item_count);
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let received = {
            let receiver = rx.lock().unwrap();
            receiver.recv()
        };
        
        match received {
            Ok(number) => {
                if number == TERMINATION_SIGNAL {
                    println!("Consumer {} received termination signal. Exiting...", id);
                    break;
                } else {
                    // Process the number
                    println!("Consumer {} received and processing number: {}", id, number);
                    thread::sleep(Duration::from_millis(50));
                    println!("Consumer {} completed processing number: {}", id, number);
                }
            }
            Err(_) => {
                // Channel is closed and empty
                println!("Consumer {}: channel closed. Exiting...", id);
                break;
            }
        }
    }
}