use std::sync::{mpsc, Arc, Mutex};

use crate::term::*;

type Job = Box<dyn FnOnce() + Send + 'static>;

/**
 * Struct defining a specific parallel building run.
 */
pub struct ParallelBuild {
    threads: Vec<std::thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
    size: usize,
}

impl ParallelBuild {
    /**
     * Create a new ParallelBuild using `size` CPUs (parallel jobs) which will have a total of
     * `total_jobs` jobs run on it.
     */
    pub fn new(size: usize, total_jobs: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver): (mpsc::Sender<Job>, mpsc::Receiver<Job>) =
            std::sync::mpsc::channel();
        let receiver: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        let completed: Arc<std::sync::atomic::AtomicUsize> =
            Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            let recv = Arc::clone(&receiver);
            let completed = Arc::clone(&completed);
            let total_jobs = total_jobs.clone();
            let handle = std::thread::spawn(move || loop {
                let comp = completed.fetch_add(0, std::sync::atomic::Ordering::SeqCst);
                if comp >= total_jobs {
                    break;
                }
                let task = match recv.lock() {
                    Ok(r) => match r.recv() {
                        Ok(v) => v,
                        Err(_) => {
                            error!("A parallel build worker failed to receive jobs. The build WILL fail past this point. Abort.");
                            panic!("Failed to receive jobs.");
                        }
                    },
                    Err(_) => {
                        error!("A parallel build worker failed to receive jobs. The build WILL fail past this point. Abort.");
                        panic!("Failed to receive jobs.");
                    }
                };
                task();
                completed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });

            threads.push(handle);
        }
        Self {
            threads,
            sender,
            size,
        }
    }

    /**
     * Submit a job to this ParallelBuild.
     */
    pub fn submit<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }

    /**
     * Waits for this ParallelBuild to finish running.
     * You MUST call this or the ParallelBuild's threads will not be joined.
     */
    pub fn wait(&mut self) {
        for _ in 0..self.size {
            self.submit(|| {});
        }

        'outer: loop {
            let mut cont: bool = false;
            for handle in &mut self.threads {
                if !handle.is_finished() {
                    cont = true;
                }
            }
            if !cont {
                break 'outer;
            }
        }
    }
}
