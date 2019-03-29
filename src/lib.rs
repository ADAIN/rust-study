pub mod structs;

pub fn add_two(a: i32) -> i32{
    a + 2
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

fn print_and_return_10(a: i32) -> i32{
    println!("Input value is {}", a);
    10
}

#[cfg(test)]
mod tests {
    use crate::structs::Rectangle;
    use crate::add_two;
    use super::*;

//    #[test]
//    #[ignore]
//    fn ignore_test(){
//
//    }

    #[test]
    fn print_and_return_will_pass(){
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
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width: 10,
            height: 10
        };

        let smaller = Rectangle{
            width: 5,
            height: 9
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4, add_two(2));
        assert_ne!(2, add_two(2));
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100(){
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
mod tests2{
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}