// 열심히 했다가 날아가서 살짝 빡침.
// 기본 추리 게임 완성 코드

// extern crate rand;  // = use rand 임. 이거 사실 왜 있는지 모르겠음.. 없어도 될것같은데. 
// 확인 결과, Rust 2018 이후에서는 필요 없는 것임이 확인됨.

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop{
        println!("Plz input your guess.");

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;}
        }
    }
}