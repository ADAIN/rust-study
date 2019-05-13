//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-13
//! Description :
//!

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32{
    f(arg) + f(arg)
}

fn returns_closure() -> Box<fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

pub fn run(){
    let answer = do_twice(add_one, 10);
    println!("{}", answer);

    let answer = do_twice(*returns_closure(), 10);
    println!("{}", answer);
}