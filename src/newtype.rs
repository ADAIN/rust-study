//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-13
//! Description :
//!

type Kilometers = i32;

type Thunk = Box<Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {

}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("GOOD"))
}

pub fn run(){
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
    takes_long_type(f);
}