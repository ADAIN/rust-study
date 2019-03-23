extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;
use std::io::{Error, ErrorKind};

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Result<Guess, io::Error> {
        if value < 1 || value > 100{
            return Result::Err(Error::new(ErrorKind::InvalidInput, "입력값은 1~100사이의 숫자이어야 합니다."));
        }

        Ok(Guess {
            value
        })
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

pub fn run(){
    let mut money: u32 = 1000;
    println!("숫자맞추기 게임에 오신걸 환영합니다.");
    println!("게임에 성공시 횟수에 따라 보상이 지급 됩니다.");
    println!("최대 5번을 시도 할 수 있고 실패시 참여금을 소진하게 됩니다.");
    println!("참여금은 100원 입니다.");

    loop{
        println!("현재 소지금은 {}원 입니다.", &mut money);
        println!("숫자를 맞춰보세요.");
        println!("숫자를 입력해 주세요.");

        // 랜덤 숫자 만들기
        let secret_number = rand::thread_rng().gen_range(1, 101);
        let mut count: u32 = 5;
        let play_money: u32 = 100;

        println!("게임을 시작 하시겠습니까?");
        eprint!("Y/N [Y]:");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("입력값 가져오기 실패.");

        let answer = answer.trim().to_uppercase();

        if !(answer.is_empty() || answer == "Y") {
            println!("최종 소지금은 {}원 입니다.\n감사합니다.", &mut money);
            break;
        }

        println!("비밀 숫자: {}", secret_number);
        println!("100원을 차감합니다.");
        money -= play_money;
        println!("현재 소지금은 {}원 입니다.", &mut money);

        loop{
            // 숫자를 입력받고 저장하기 위한 변수
            let mut guess = String::new();

            eprint!("숫자입력:");

            // 숫자를 입력받는다.
            io::stdin()
                .read_line(&mut guess)
                .expect("입력값 가져오기 실패.");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // 입력받은 숫자를 출력한다.
            println!("입력한 숫자: {}", guess);

            let guess = match Guess::new(guess) {
                Ok(ok) => ok.value(),
                Err(e) => {
                    println!("입력값 오류 {:?}", e);
                    continue
                },
            };

            match guess.cmp(&secret_number){
                Ordering::Less => {
                    println!("값이 너무 작습니다.");
                    count -= 1;
                },
                Ordering::Greater => {
                    println!("값이 너무 큽니다.");
                    count -= 1;
                },
                Ordering::Equal => {
                    println!("값이 일치 합니다.");
                    let prize: u32 = play_money * (count + 1);
                    println!("축하합니다! 상금은 {}원 입니다.", &prize);

                    money += prize;
                    break;
                }
            }

            if count < 1 {
                println!("실패 하셨습니다.");
                break;
            }
        }
    }
}
