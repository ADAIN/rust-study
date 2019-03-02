extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("숫자를 맞춰보세요.");
    println!("숫자를 입력해 주세요.");

    // 랜덤 숫자 만들기
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("비밀 숫자: {}", secret_number);

    // 숫자를 입력받고 저장하기 위한 변수
    let mut guess = String::new();

    // 숫자를 입력받는다.
    io::stdin()
        .read_line(&mut guess)
        .expect("입력값 가져오기 실패.");

    // 입력받은 숫자를 출력한다.
    println!("입력한 숫자: {}", guess);
}
