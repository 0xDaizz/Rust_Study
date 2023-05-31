// cargo-deps: rand = "0.8.5"

// 기본 추리 게임을 업그레이드 한 버전.

// 7번 이내에 맞추지 못하면 게임을 지는 방식으로 바꿔보겠음.



use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number between 1~100!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    let mut chance = 7;


    while chance > 0 {
        println!("---------------------\nPlz input your guess.");

        println!("You have {} guess(es) left.", chance);

     

        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("Invalid number!");
                    continue;
                }
                num
            }
            Err(_) => continue,
        };

        chance -= 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("Correct! You win!!"); return;}
        }
    }

    println!("You lose. the correct answer was {}.", secret_number);
    
}