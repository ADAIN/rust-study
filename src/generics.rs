use std::fmt::Display;
use std::fmt;

fn largest<T>(list: &[T]) -> T where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &number in list.iter() {
        if number > largest{
            largest = number;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summarizable{
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct GO{

}

impl Summarizable for GO {

}

impl Display for GO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GOGOGO")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tweet ({}, {})", self.username, self.content)
    }
}

//pub fn notify<T: Display + Summarizable>(item: T) {
pub fn notify<T>(item: T) where T: Display + Summarizable{
    println!("Wow {}", item.summary());
    println!("D {}", item);
}

pub fn run(){
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);

    let a = Point{x: 5, y: 10};
    let b = Point{x: 1.0, y: 1.0};
    let c = Point{x: 1.0, y: 1};
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("{:?}", p3);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
    println!("{}", tweet);

    let go = GO {

    };
    println!("{}", go.summary());
    notify(go);
    notify(tweet);
}