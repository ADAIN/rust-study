//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-08
//! Description :
//!
use std::sync::{Mutex, Arc};
use std::thread;

pub fn run() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", *m.lock().unwrap() + 10);
    println!("m = {:?}", m);

    run_thread();
}

fn run_thread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
