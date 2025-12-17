use std::{fs, sync::{Arc, Mutex}};
use thread_pool::ThreadPool;
use progress::*;
use processor::process_file;

mod thread_pool;
mod processor;
mod analyzers;
mod progress;
mod errors;
mod types;

fn main() {
    let pool = ThreadPool::new(8);
    let progress: SharedProgress = Arc::new(Mutex::new(ProgressTracker::default()));

    let paths = fs::read_dir("books")
        .expect("books directory missing")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect::<Vec<_>>();

    for path in paths {
        let progress = Arc::clone(&progress);
        let path_str = path.to_string_lossy().to_string();

        pool.execute(move || {
            let analysis = process_file(&path_str);

            let mut p = progress.lock().unwrap();
            if analysis.errors.is_empty() {
                p.completed += 1;
            } else {
                p.failed += 1;
            }

            println!("Processed: {}", analysis.filename);
        });
    }

    pool.shutdown();

    let p = progress.lock().unwrap();
    println!("Completed: {}, Failed: {}", p.completed, p.failed);
}
