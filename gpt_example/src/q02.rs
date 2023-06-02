// cargo-deps: rand = "0.8.5"
#![allow(dead_code)]
#![allow(unused)]
// 두 번쨰 10문제

// 1. 문자열 피라미드

fn str_pyramid(s: &str) {
    for i in 0..s.len() {
        println!("{}", &s[0..i]);
    }

    for i in (s.len()-1)..0 {
        println!("{}", &s[0..i]);
    }
}



// 2. 공백 제거

fn no_blank(s: &str) -> String {
    s.split_whitespace().collect()
}


// 3. 원소 합계

fn sum_vec(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}


// 4. Pallindrome

fn is_p(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    reversed.to_string() == s
}


// 5. 가장 긴 단어 찾기

fn longest_word(s: &str) -> &str {
    s.split_whitespace().max_by_key(|x| x.len()).expect("Empty string")
}


// 6. 중복 제거

fn eliminate_twice(v: Vec<i32>) {
    // v.iter().filter(......`

        //모르겠다 시벌 ㅋㅋ
}


// 7. Random vector

use std::f64::consts::PI;

use rand::Rng;

fn random_vec(n: u32) -> Vec<i32> {
    let mut random_vector = Vec::new();
    let mut i = 0;

    while i < n {
        random_vector.push(rand::random::<i32>());
        i += 1;
    }
    random_vector
}


// 8. ***

fn asterisk(n: usize) {
    for i in 1..=n {
        println!("{}", "*".repeat(i));
    }

    for i in n..1 {
        println!("{}", "*".repeat(i-1));
    }
}


// 9. 구조체 메서드

struct Circle {
    radius: f64
}

impl Circle {
    fn p(&self) -> f64 {
        PI * 2.0 * &self.radius
    }

    fn area(&self) -> f64 {
        PI * (&self.radius).powf(2.0)
    }
}

// 숫자도 정수형이랑 소수점형이랑 따로니까 ㄹㅇ 개불편하네...
// 심지어 Math 라이브러리 그런 거 없나? JS TS 하다가 이거하니까 암걸릴것같음


// 10. ERR

fn err_(s: &str) -> i32 {
    s.parse::<i32>().expect("Not a integer input!!")
}