use parallel_file_processor::thread_pool::ThreadPool;
use std::sync::{Arc, Mutex};

#[test]
fn thread_pool_executes_jobs() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        pool.execute(move || {
            *c.lock().unwrap() += 1;
        });
    }

    pool.shutdown();

    assert_eq!(*counter.lock().unwrap(), 10);
}
