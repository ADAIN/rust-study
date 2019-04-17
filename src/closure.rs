//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-04-15
//! Description :
//!

use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32, u64) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32, u64) -> u32 {
    fn new (calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32, secs: u64) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg, secs);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let max_secs = 2;
    let mut cache = Cacher::new(|num, mut secs| {
        println!("calculation slowly... secs: {}, max secs: {}", secs, max_secs);
        if secs > max_secs {
            secs = max_secs;
            println!("secs[{}] is bigger than max_secs[{}]", secs, max_secs);
        }
        thread::sleep(Duration::from_secs(secs));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cache.value(intensity, 10));
        println!("Next, do {} situps!", cache.value(intensity, 1));
    }else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }else{
            println!("Today, run for {} minutes!", cache.value(intensity, 1));
        }
    }
}

pub fn run(){
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let mut x = 4;
    let mut equal_to_x = |z| {
        x = x + 1;
        x
    };
    let y = 4;
    println!("{:?}", equal_to_x(y));
    println!("{:?}", equal_to_x(y));

    x = 10;
    println!("{}", x);

    let a = vec![1, 2, 3];
    let equal_to_a = move |z| z == a;
    println!("can't use x here: {:?}", a);
    let y = vec![1, 2, 3];
    assert!(equal_to_a(y));
}

