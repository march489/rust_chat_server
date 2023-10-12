use rand;
use std::thread::{spawn, JoinHandle};
use std::time::SystemTime;

fn _not_a_random_number() -> u8 {
    let now = SystemTime::now();
    let duration = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let nanos: u128 = duration.as_nanos();
    (nanos % 10) as u8
}

fn main() {
    let mut join_handles: Vec<(u8, JoinHandle<u8>)> = Vec::new();
    for thread_number in 0..4 {
        let join_handle = spawn(move || loop {
            let num: u8 = rand::random::<u8>() % 10 as u8;
            eprintln!("thread {thread_number}:  {num}");
            let _ = 100 / num;
            if num == 1 {
                return thread_number;
            }
        });
        join_handles.push((thread_number, join_handle));
    }
    for (thread_number, join_handle) in join_handles {
        match join_handle.join() {
            Ok(n) => {
                eprintln!("thread {thread_number}: joined successfully with value {n}");
            }
            Err(e) => {
                eprintln!("thread {thread_number}: joined with error {e:?}");
            }
        }
    }
}
