// Note: this threadpool implementation is pretty much taken from the rust language book
use crate::log::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// type alias for a job to execute in a thread
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A message sent to a worker
pub enum Msg {
    Exec(Job),
    Terminate,
}

/// A worker in the threadpool recieving jobs and executing them
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// construct a new worker
    /// # Arguments
    /// *`id`: the id this worker uses for logging purposes
    /// *`reciever`: the recieving end of a channel
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Msg>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            let mut logger: Logger = Logger::new();
            logger.set_term(btui::Terminal::default());
            match job {
                Msg::Exec(task) => {
                    logger.log(format!("Worker {}: executing task...", id), LogType::Log);
                    task();
                }
                Msg::Terminate => {
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

/// a thread pool with a set amount of threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Msg>,
}

impl ThreadPool {
    /// create a new threadpool with `size` many threads
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }
        ThreadPool { workers, sender }
    }

    /// give a job to execute to the threadpool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        match self.sender.send(Msg::Exec(job)) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("failed to execute task: {}", e);
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Msg::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
