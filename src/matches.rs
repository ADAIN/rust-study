enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u32{
    match coin {
        Coin::Penny => {
            println!("WOO!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::Alabama => {
                    println!("Alabama");
                },
                UsState::Alaska =>{
                    println!("Alaska");
                },
            }
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

pub fn run(){
    println!("Penny : {}", value_in_cents(Coin::Penny));
    println!("Nickel : {}", value_in_cents(Coin::Nickel));
    println!("Dime : {}", value_in_cents(Coin::Dime));
    println!("Quarter : {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("Quarter : {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let ten = Some(10);
    let eleven = plus_one(ten);
    let none = plus_one(None);

    if let Some(value) = eleven {
        println!("{}", value);
    }

    if let Some(value) = none {
        println!("출력 안됩니다.... {}", value);
    }

    if let None = none {
        println!("None!");
    }

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}