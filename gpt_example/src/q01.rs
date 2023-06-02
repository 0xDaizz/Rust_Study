#![allow(dead_code)]
#![allow(unused)]
//q01

// 1. 문자열 반전

use std::{usize, fmt::Error};

fn reverse_string(input: String) -> String {
    input.chars().rev().collect()

    // chars() : 한 글자씩 잘라줌
    // rev() : 뒤집음. 안 뒤집는 건 iter()
    // collect() : iter(rev) 반복자 세팅한 걸 모아줌.
}


// 2. fibonacci sequence

fn fibo(n: u32) -> Vec<u32> {
    let mut fibonacci: Vec<u32> = vec![1, 1];

    return match n {
        0 => vec![],
        1 => vec![1],
        2 => fibonacci.clone(), //clone() 해 줘서 벡터의 복사본을 반환하는 게 더 안전하다.
        _ => {
            for i in 2..n {
                fibonacci.push(fibonacci[(i-2) as usize] + fibonacci[(i-1) as usize]);
                // 인덱싱은 usize 타입을 써야 하는데, 이런 경우 as로 해주면 된다!!
            }
            fibonacci
        }
    }
}


// 3. 오름차순 정리

fn arrange_vec(vector: Vec<i32>) -> Vec<i32> {
    let mut v = vector.clone();

    for i in 0..(v.len()) {             // 범위의 a..b 는 a 포함, b 제외이다. 그래서 -1을 할 필요가 없음
        for j in (i+1)..(v.len()) {
            if v[i] > v[j] {
                (v[i], v[j]) = (v[j], v[i]);
            }
        }
    }
    v
}

// 모범답안

pub fn answer_vec(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();   // sort() : 오름차순으로 정리
    v
}




// 4. 짝수 합계

fn sum_even (v: Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in v {            // v 안에 있는 모든 내용물에 대해 루프를 돌리려면 i in v가 맞음.
        if i % 2 == 0 {
            sum += i
        }
    }
    sum as i32
}

// iterator를 사용한 모범 답안
fn sum_even_iter(v : Vec<i32>) -> i32 {
    v.iter().filter(|&x| x % 2 == 0).sum()
}





// 5. Factorial

fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;

    for i in 1..=n {        // 1~n 까지 n을 포함하는 범위는 1..n 이 아니라 1..=n 임!!
        result *= i;
    }
    result
}

// 모범답안

fn fac(n: u32) -> u32 {
    (1..=n).product()       // 아니 ㅅㅂ 이렇게 간단하게 할 수 있던거임..?
}





// 6. 문자열에서 단어 세기

fn count_words(s: String) -> u32 {
    s.split(" ").collect::<String>().len() as u32
}
// split(" ") 를 쓰는 방법은, 빈 문자열 혹은 띄어쓰기 두 개 이상을 경우 오류 발생할 수도.
// split_whitespace()를 쓰는 게 더 안전
// 그리고 개수 셀 때는 count()만 하면 되는듯 ㅋㅋ

// 모범답안
fn count_ans(s: &str) -> usize {
    s.split_whitespace().count()
}



// 7. Struct, method

struct Rectangle {
    garo: u32,
    sero: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        &self.garo * &self.sero
    }
}

fn area_cal() {
    let rect1 = Rectangle {garo : 4, sero : 5};

    let area = rect1.area();
    println!("{area}");
}

// 굳. 구조체 어려웠는데 잘 한듯.


// 8. Option 사용

fn use_option(v: Vec<String>) -> Option<String>{
    let result = v.get(0);
    result.cloned()
}

// 모범답안

fn useoption(v: &[String]) -> Option<&String> {     // [String] 이렇게 표기하면 되나보당.
    v.get(0)    // 굳이 변수에 할당하고 클론하고 지랄할 필요가 없었는듯. 
    // 핵심은 아마도 타입 표현 아니었을까. 
    // 처음 변수 v 대입은 : 벡터를 참조해 가져온 거니까 &[].
    // 출력은 : v.get(0) 으로 문자열을 참조한 거니까 &String으로 가져옴.

}


// 9. Error handling

fn use_option_two(s: String) -> i32 {
    let s_parsed = s.parse::<i32>();
    let s_num = match s_parsed {
        Ok(v) => return v,
        Err(e) => panic!("Cannot parse into number")
    };
}

// 이 문제에서는 에러를 반환하는 게 막혔었는데...
// Ok<T>(t) 혹은 Err(e) 의 타입은 : Result<T, std::num::ParseIntError> 임. 
// 물론 반환하는 에러의 타입마다 달라지겠지만..

// 모범답안
fn ans_err(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}



// 10. Borrowing

// fn move_vec<T>(a: &Vec<T>, b: &mut Vec<T>) {

//     let _a = a.clone();
//     let _b = b.;

//     let last_a = _a.get(_a.len()-1).unwrap();
//     let last_aa = last_a;

//     let new_b = vec![last_aa].append(_b);

//     new_b

//     // 시발 존나복잡해 어떡하란겨
// }

// 이건 뭐 답이 없더라
// 차근차근 보자.

// 벡터 두 개 a, b (각각 타입은 Vec<T>)
// a의 소유권을 함수가 뺏어오면 안 됨.
// a의 마지막 요소를 가져와, b의 맨 앞 요소로 삽입해야 함.

// 즉, a는 수정되지 않고 참조만 할 거고,
// b는 참조를 해서 수정까지 할 거다.

// 따라서 파라미터 결정 : 
// a: &Vec<T>, b: &mut Vec<T>

// T가 clone 가능한 타입임을 명시.

// 모범답안

fn ans_ten<T: Clone>(a: &Vec<T>, b: &mut Vec<T>) {
    if let Some(last) = a.last() {
        b.insert(0, last.clone())
    }
}

// 뭔가 굉장히 함축적임 ㄷㄷ..

// if let A = B {C} 는
// 
// let a = match B {
//      A => C
//      _ => ()
// }
// a

// 메서드 배워가기
// last() => Some(v) 혹은 빈 벡터면 None
// insert(a, b) => 인덱싱 a 자리에 b를 끼워넣는다.