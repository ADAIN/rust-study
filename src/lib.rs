//! # rust study
//!
//! 이것은 연습용
//!
//! 이것저것 테스트하는 코드
//!

use std::sync::atomic::Ordering::AcqRel;

pub mod structs;

/// Adds two to the number given.
///
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(7, rust_study::add_two(five));
/// ```
///
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct AveragedCollection{
    list: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0
        }
    }
    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self){

    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

fn print_and_return_10(a: i32) -> i32 {
    println!("Input value is {}", a);
    10
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft{}
struct PendingReview {}
pub struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}


impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<State>{
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

impl State for Published{
    fn request_review(self: Box<Self>) -> Box<State>{
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::add_two;
    use crate::structs::Rectangle;

    //    #[test]
    //    #[ignore]
    //    fn ignore_test(){
    //
    //    }

    #[test]
    fn print_and_return_will_pass() {
        let value = print_and_return_10(4);
        assert_eq!(10, value);
    }

    //    #[test]
    //    #[ignore]
    //    fn print_and_return_will_fail(){
    //        let value = print_and_return_10(4);
    //        assert_eq!(5, value);
    //    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };

        let smaller = Rectangle {
            width: 5,
            height: 9,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(2, add_two(2));
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[cfg(test)]
mod tests2 {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
