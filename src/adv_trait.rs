//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-10
//! Description :
//!

use std::ops::Add;
use std::fmt::{Display, Formatter, Error};

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, PartialEq)]
struct Counter{
    count: i32
}

impl Add for Counter {
    type Output = Counter;

    fn add(self, other: Counter) -> Self::Output {
        Counter {
            count: self.count + other.count
        }
    }
}

impl Iterator for Counter{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item>{
        Some(self.count)
    }
}

#[derive(PartialEq, Debug)]
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run(){
    let mut counter = Counter{ count: 1 };
    assert_eq!(counter.next().unwrap(), 1);

    let counter2 = Counter{count: 2};
    let counter3 = counter + counter2;
    assert_eq!(counter3.count, 3);
    assert_eq!(counter3, Counter{ count: 3});

    let a = Millimeters(10);
    let b = Meters(1);
    assert_eq!(a + b, Millimeters(1010));

    let human = Human;
    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);

    println!("{}", Dog::baby_name());
    println!("{}", <Dog as Animal>::baby_name());

    let point = Point {x: 10, y: 100};
    point.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}