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

        // Wrap receiver in Arc<Mutex> so it can be shared among workers
        let receiver = Arc::new(Mutex::new(receiver));

        // Create and store workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Box the closure and send it as a NewJob message
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send terminate message to every worker
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // Wait for all workers to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
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
        // Spawn a thread that loops, receives messages, and processes jobs
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} picked up a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} received terminate signal.", id);
                    break;
                }
            }
        });

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
    // ThreadPool is dropped here, triggering Drop which sends Terminate to all workers
    // and joins their threads — guaranteeing all tasks finish before the program exits.
}
