//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-09
//! Description :
//!
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum MyMsg {
    Hello {id: i32}
}

pub fn run(){
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("y = {:?}", y),
        _ => println!("default case, x = {:?}, y = {:?}", x, y)
    }

    println!("x = {:?}, y = {:?}", x, y);

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("any"),
    }

    let x = 5;

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("any"),
    }

    let x = 'z';
    match x {
        'a' ... 'j' => println!("a ~ j"),
        'k' ... 'z' => println!("k ~ z"),
        _ => println!("something else"),
    }

    let p = Point {
        x: 0,
        y: 7
    };

    let Point {
        x: a,
        y: b
    } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point {x, y} = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("neither ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("Quit");
        },
        Message::Move { x, y } => {
            println!("{}, {}", x, y);
        },
        Message::Write(text) => println!("text : {}", text),
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("change the color to hue {}, saturation {}, and value {}", h, s, v);
        },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("change the color to red {}, green {}, blue {}", r, g, b);
        },
    }

    let points = vec![
        Point { x: 0, y: 0},
        Point { x: 1, y: 5},
        Point { x: 10, y: -3},
    ];

    let sum_of_squares: i32 = points.iter().map(|Point {x, y}| x * x + y * y).sum();
    println!("{}", sum_of_squares);

    let ((fleet, inches), Point {x, y}) = ((3, 10), Point {x: 3, y: -10});

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
          println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("{}", setting_value.unwrap());

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    let s = Some(String::from("Hello"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let point = Point {x: 0, y: 0};
    match point {
        Point {x, ..} => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("{}, {}", first, last);
        }
    }

    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => *name = String::from("AA"),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = MyMsg::Hello {id: 5};

    match msg {
        MyMsg::Hello {id: a @ 3...7} => {
            println!("HI: {}", a);
        },
        MyMsg::Hello {id: 10 ... 13} =>{
            println!("Found an id in another range");
        },
        MyMsg::Hello {id} =>{
            println!("{}", id);
        }
    }
}