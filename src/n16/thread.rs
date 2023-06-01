// Thread 사용법

// 새롭게 Thread를 생성하려면 thread::spawn 호출 -> 
// 새로운 스레드 내에서 실행하기 원하는 클로저를 넘긴다.

// 우선 크레이트 가져오기.

use std::thread;
use std::time::Duration;

fn thread_spawn() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from A", i);
            thread::sleep(Duration::from_millis(1));    // 새로운 스레드 만들어서, 1밀리초마다 메시지 출력 
        }
    });

    for i in 1..5 {
        println!("Number {} from B", i);
        thread::sleep(Duration::from_millis(1));    // 대조군.
    }
}
/*
결과:
Number 1 from B
Number 1 from A
Number 2 from B
Number 2 from A
Number 3 from B
Number 3 from A
Number 4 from B
Number 4 from A
 */



pub fn main() {
    thread_spawn();
}