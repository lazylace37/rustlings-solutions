// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for sleep_ms in (0..1000).step_by(100) {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(sleep_ms));
            // TODO: You must take an action before you update a shared value
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }

    let start = SystemTime::now();
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        let jobs_completed = status.lock().unwrap().jobs_completed;
        let now = SystemTime::now();
        println!(
            "jobs completed {} {} ms",
            jobs_completed,
            now.duration_since(start).unwrap().as_millis()
        );
    }

    // or
    // while status.lock().unwrap().jobs_completed < 10 {
    //     println!("Waiting for threads to finish....");
    //     thread::sleep(Duration::from_millis(100));
    // }
    // println!("All threads done.");
}
