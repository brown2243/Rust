//  사용자 입력을 받고 결과값을 표시하기 위해서는 io (input/output) 라이브러리를 스코프로 가져와야 합니다.
//  io 라이브러리는 std 라고 불리는 표준 라이브러리에 있습니다.
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // print
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");
        // mut string 변수 선언
        let mut guess = String::new();
        // input 받는 코드
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // 출력
        println!("You guessed: {}", guess);
        // check
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
