/*

1. chance 변수의 이름을 remaining_guesses로 변경하였습니다. 이를 통해 이 변수가 얼마나 많은 추측 기회가 남았는지 더 명확하게 나타낼 수 있습니다.

2. println! 명령어를 사용하여 사용자에게 입력을 요청하는 문구를 좀 더 친절하게 변경하였습니다. "Plz" 대신 "Please"를 사용하였습니다.

3. 사용자가 범위 내의 숫자를 입력했을 경우에만 추측 횟수가 줄어들도록 수정하였습니다. 이를 위해 match 문에 조건을 추가하였습니다.

4. 범위 외의 숫자를 입력했을 경우에 대한 오류 메시지를 좀 더 친절하게 만들었습니다.

이러한 변경사항은 코드의 가독성을 높이며, 이해하기 쉬운 코드를 작성하는 것이 좋은 코딩 습관입니다. */


use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1~100!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut remaining_guesses = 7;

    while remaining_guesses > 0 {
        println!("---------------------\nPlease input your guess.");
        println!("You have {} guess(es) left.", remaining_guesses);

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        match guess.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => {
                println!("You guessed: {}", num);
                remaining_guesses -= 1;
                match num.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("Correct! You win!!");
                        return;
                    }
                }
            },
            Ok(_) => {
                println!("Invalid number! Please enter a number between 1 and 100.");
            },
            Err(_) => {
                println!("Failed to parse guess. Please enter a number.");
            }
        }
    }

    println!("You lose. The correct answer was {}.", secret_number);
}
