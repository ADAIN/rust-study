enum IpAddrKind{
    V4,
    V6
}

enum IpAddrEnum{
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr{
    kind: IpAddrKind,
    address: String
}

pub fn run(){
    let ip4: IpAddrKind = IpAddrKind::V4;
    let ip6: IpAddrKind = IpAddrKind::V6;

    println!("{} {}", route(ip4), route(ip6));
    println!("{}", route(IpAddrKind::V4));

    let ip_addr: IpAddr = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.11.5.1")
    };

    println!("{} {}", ip_addr.address, route(ip_addr.kind));

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));
    if let IpAddrEnum::V4(a, b, c, d) = home {
        println!("IP: {}.{}.{}.{}", a, b, c, d);
    }

    match home {
        IpAddrEnum::V4(a, b, c, d) => println!("match IP: {}.{}.{}.{}", a, b, c, d),
        IpAddrEnum::V6(addr) => println!("{}", addr),
    }
    match loopback {
        IpAddrEnum::V4(a, b, c, d) => println!("match IP: {}.{}.{}.{}", a, b, c, d),
        IpAddrEnum::V6(addr) => println!("{}", addr),
    }

    let message = Message::Quit;
    let message2 = Message::ChangeColor(12, 22, 255);
    let message3 = Message::Move {x: 12, y: 30};
    let message4 = Message::Write(String::from("HI"));
    let mut message_vec: Vec<Message> = Vec::new();
    message_vec.push(message);
    message_vec.push(message2);
    message_vec.push(message3);
    message_vec.push(message4);

    for item in message_vec.iter(){
        match item {
            Message::Quit => println!("QUIT"),
            Message::ChangeColor(r, g, b) => println!("{}, {}, {}", r, g, b),
            Message::Move {x, y} => println!("x:{}, y:{}", x, y),
            Message::Write(a) => println!("{}", a),
        }
    }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

    if absent_number == Option::None {
        println!("NOOOOOO!!!!");
    }

    let x: i8 = 5;
    let y: Option<i8> = Some(15);

    if let Option::Some(value) = y {
        println!("{} + {} = {}", x, value, x + value);
    }
}

fn route(ip_type: IpAddrKind) -> &'static str {
    match ip_type {
        IpAddrKind::V6 => {
            return "V6";
        },
        IpAddrKind::V4 =>{
            return "V4";
        }
    }
}
