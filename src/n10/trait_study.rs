#![allow(dead_code)]
#![allow(unused_variables)]

// trait 배우기 전에, struct 복습

struct ThisIsAStruct {
    one: String,
    two: i32,
    three: bool,
}

// 인스턴스 생성
fn _instance() {
    let mut instance_one = ThisIsAStruct {
        one: String::from("ONE"),
        two: 2,
        three: true,
    };
}

// 인스턴스의 반환
fn return_instance(one: String, two: i32) {
    ThisIsAStruct {
        one,    // one = one, 과 동일!
        two,
    }
}








fn main() {}