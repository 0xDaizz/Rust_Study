// cargo-deps: rand = "0.8.5", itertools,
#![allow(dead_code)]
#![allow(unused)]
// 두 번쨰 10문제

// 1. 문자열 피라미드

fn str_pyramid(s: &str) {
    let len = s.chars().count();    // s.len()이라고 하면, UTF-8 하에서 바이트 단위로 세기 때문에 에러날 수 있음
    // chars - count 메소드 체인을 쓰는 방법이 더 낫다.
    
    for i in 0..len {
        println!("{}", &s[0..i]);
    }

    for i in (0..len-1).rev() {       // 이것 또한 역순을 원하면 정배열로 쓴 다음 rev()해줘야 한다.
        println!("{}", &s[0..i]);
    }
}



// 2. 공백 제거

fn no_blank(s: &str) -> String {
    s.split_whitespace().collect::<String>()
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

        // 해쉬셋이라는 걸 쓰는데, 아직 ㅈ도 모르겠음 ㅋㅋㅋ
}
// 일단 답지 올려둠.

use std::collections::HashSet;

fn remove_duplicates<T: std::cmp::Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    vec.into_iter().filter(|x| seen.insert(x.clone())).collect()
}
// 지랄맞다 ㅋㅋㅋㅋㅋ
// 그런데, 서드파티 모듈을 쓰면 낫지 않을까 했는데 이게 있었음

// 서드파티 크레이트 "itertools" 를 사용한 풀이

use itertools::Itertools; // add to your Cargo.toml

fn eliminate_duplicates(vec: Vec<i32>) -> Vec<i32> {
    vec.into_iter().unique().collect()
}

// 후... 훨씬 낫다 진짜.



// 7. Random vector

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

// 모범답안

fn ans_rand(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen()).collect()
}




// 8. ***

fn asterisk(n: usize) {
    for i in 1..=n {
        println!("{}", "*".repeat(i));
    }

    for i in (1..n).rev() {             // 역순으로 하려면 rev()를 써줘야한다.
        println!("{}", "*".repeat(i));
    }
}


// 9. 구조체 메서드

use std::f64::consts::PI;

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