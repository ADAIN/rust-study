#[derive(Debug)]
struct User {
    username: String,
    age: i8,
    gender: char,
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn get_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    pub fn get_size(&self) -> u32 {
        self.height * self.width
    }

    pub fn can_hold(&self, target: &Rectangle) -> bool {
        self.width > target.width && self.height > target.height
    }
}

struct Color(u8, u8, u8);

pub fn run() {
    let user = User {
        username: String::from("SungYong Jang"),
        age: 40,
        gender: 'M',
    };

    println!("{} {} {}", user.username, user.age, user.gender);
    println!("{:?}", user);
    println!("{:#?}", user);

    let color = Color(255, 255, 0);
    println!("{}, {}, {}", color.0, color.1, color.2);

    let rect = Rectangle {
        width: 10,
        height: 5,
    };

    println!("{}", rect.get_size());

    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 10,
    };
    let rect3 = Rectangle {
        width: 45,
        height: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::get_square(10);
    println!("Square: {:?}", square);
}
