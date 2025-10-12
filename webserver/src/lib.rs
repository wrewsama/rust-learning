use std::{sync::{mpsc, Arc, Mutex}, thread::{self}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
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
        Ok(ThreadPool{ workers, sender: Some(sender) })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shutdown {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, recver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            println!("worker {id} starting");
            loop {
                let msg = recver.lock().unwrap().recv();
                match msg {
                    Ok(job) => {
                        println!("worker {id} received job");
                        job();
                    }
                    Err(_) => {
                        println!("worker {id} disconnected");
                        break;
                    }
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
