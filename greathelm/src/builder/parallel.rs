use std::sync::{mpsc, Arc, Mutex};

use crate::term::error;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ParallelBuild {
    threads: Vec<std::thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
    size: usize,
}

impl ParallelBuild {
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
                            error("A parallel build worker failed to receive jobs. The build WILL fail past this point. Abort.".into());
                            panic!("Failed to receive jobs.");
                        }
                    },
                    Err(_) => {
                        error("A parallel build worker failed to receive jobs. The build WILL fail past this point. Abort.".into());
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

    pub fn submit<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }

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
