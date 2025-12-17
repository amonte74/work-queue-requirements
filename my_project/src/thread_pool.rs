// src/thread_pool.rs
use std::{
    sync::{Arc, Mutex, Condvar},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    queue: Arc<(Mutex<Vec<Job>>, Condvar)>,
    shutdown: Arc<Mutex<bool>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let queue = Arc::new((Mutex::new(Vec::<Job>::new()), Condvar::new()));
        let shutdown = Arc::new(Mutex::new(false));

        let mut workers = Vec::new();

        for _ in 0..size {
            let q = Arc::clone(&queue);
            let s = Arc::clone(&shutdown);

            workers.push(thread::spawn(move || loop {
                let job = {
                    let (lock, cvar) = &*q;
                    let mut jobs = lock.lock().unwrap();

                    while jobs.is_empty() && !*s.lock().unwrap() {
                        jobs = cvar.wait(jobs).unwrap();
                    }

                    if *s.lock().unwrap() {
                        return;
                    }

                    jobs.pop()
                };

                if let Some(job) = job {
                    job();
                }
            }));
        }

        Self { workers, queue, shutdown }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let (lock, cvar) = &*self.queue;
        lock.lock().unwrap().push(Box::new(f));
        cvar.notify_one();
    }

    pub fn shutdown(self) {
        *self.shutdown.lock().unwrap() = true;
        self.queue.1.notify_all();

        for worker in self.workers {
            let _ = worker.join();
        }
    }
}
