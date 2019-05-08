//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-04-29
//! Description :
//!
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::mpsc;

pub fn channel_run(){
    let (tx, rx) = channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = channel();
    let tx_clone = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
       let vals = vec![
           String::from("more"),
           String::from("messages"),
           String::from("for"),
           String::from("you"),
       ];

        for val in vals {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Get: {}", received);
    }

}

pub fn run(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        for i in 1..11{
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..21{
            println!("hi number {} from the spawned2 thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    handle2.join().unwrap();
}