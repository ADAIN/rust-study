//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-09
//! Description :
//!
use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

pub fn run(){
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("{:?}, {:?}", r1, r2);

    let address = 0x012345usize;
    let r = address as *const i32;
    println!("{:?}", r);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("C: {}", abs(-3));
    }
    println!("{}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

}

#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("Called from C!");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32){
    unsafe {
        COUNTER += inc;
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

unsafe trait Foo {

}

unsafe impl Foo for i32 {

}