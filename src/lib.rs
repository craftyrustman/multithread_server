use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver
        ) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 1..=size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        };
        ThreadPool { 
            workers, 
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move||loop{
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("worker {id} got a job!");
            job()
        });
        Worker { id, thread }
    }
}