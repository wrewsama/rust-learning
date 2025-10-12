use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<Self, &'static str> {
        if size <= 0 {
            return Err("size must be >= 1");
        }
        let (sender, receiver) = mpsc::channel();
        let receiver_arc = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver_arc)));
        }
        Ok(ThreadPool{ sender})
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {}

impl Worker {
    fn new(id: usize, recver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        thread::spawn(move || {
            loop {
                let job = recver.lock().unwrap().recv().unwrap();
                println!("worker {id} received job");
                job();
            }
        });

        Worker {}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
