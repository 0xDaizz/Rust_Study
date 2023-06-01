#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// Lifetime

// 이것 또한 제네릭의 일부. 참조자가 유효한 범위를 정하는 데 중요!

use std::fmt::Display;

fn lifetime_comment() {

    // r의 생존 범위 : 'a
    // x의 생존 범위 : 'b

    // " 'x' does not live long enough" 
    let r;         // -------+-- 'a
                   //        |
    {              //        |
        let x = 5; // -+-----+-- 'b
        r = &x;    //  |     |
    }              // -+     |
                   //        |
    // println!("r: {}", r); |      // Compile 오류 막기 위해 주석처리.
                   //        |
                   // -------+
}

// 아래의 경우, 반환되는 &str가 x인지 y인지 컴파일러가 알 수 없기 때문!
// (사실, if 절의 내용이 참이면 x를 참조하고, 아니면 y를 참조하기 때문에 우리도 모름 ㅋㅋ)
// 라이프타임 명시 : 'a 를 일반적으로 사용.
// &i32 : 그냥 참조
// &'a i32 : 'a 라이프타임을 명시한 참조
// &'a mut i32 : 'a 라이프타임을 명시한 가변 참조!

// 하나의 변수에 'a 쓰는 건 사실 큰 의미 없음.
// 둘 이상의 변수에 사용되어서 '같은 라이프타임을 가진다!!' 라고 명시하는 의미를 가진다.

// 따라서 아래의 함수는 x, y, 반환값이 같은 라이프타임 기간을 공유한다는 조건을 명시해야 함!

fn lifetime_ex1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {x} else {y}
}
// lifetime 또한 제네릭이기 때문에, 파라미터 전에 <'a> 이렇게 명시해줘야 함!


// Struct 구조체 상에서의 라이프타임 도입

struct II<'a> {
    name: &'a str,
}


// Lifetime elision rules : 라이프타임 생략 규칙.
// 명시적으로 라이프타임을 표기하지 않았을 때, 다음 3가지 중 하나에 해당한다면 
// 컴파일러가 자동으로 라이프타임을 추론해 컴파일한다.
// 만약, 이 3가지 조건을 체크했는데도 라이프타임을 알 수 없는 입/출력이변수가 있다면 컴파일 에러.

// 1. 각각의 참조자 파라미터는 각자의 고유한 라이프타임을 갖는다.
// 즉, 입력 참조 파라미터가 2개인 함수는 2개의 서로 별개의 라이프타임을 가짐.

// 2. 만약 단 하나의 라이프타임 파라미터만 있는 경우, 
// 그 라이프타임을 모든 출력 라이프타임 파라미터에 적용한다.

// 3. 만약 여러 개의 입력 파라미터가 있는데, 메소드라서 그 중 하나가 &self 혹은 &mut self인 경우, 
// 그것의 라이프타임을 모든 출력 라이프타임 파라미터에 적용한다.

// 즉...

fn it_is_ok(input: &str) -> &str {input}

// 위 함수는 입력 라이프타임 파라미터가 하나이기 때문에, 모든 출력 라이프타임 파라미터에 적용된다.
// 즉, 다음과 같음.

fn _it_is_ok<'a>(input: &'a str) -> &'a str {input}


// 그렇지만 아래의 코드는 입력 라이프타임 파라미터가 2개이고, 메서드가 아니라서 &self도 없다. 즉, 

fn need_to_note_lifetime<'a, 'b> (input_a: &'a str, input_b: &'b str) -> &'a str {input_a}

// 이렇게 작성해 주어야 한다.



// Method 정의 내의 라이프타임

impl<'a> II<'a> {
    fn three(&self) -> i32 {3}   // 첫 번째 규칙에 의해, &'a self 로 명명할 필요가 없음
}


// static 라이프타임
fn static_lifetime_ex() {
    let a: &'static str = "hello";
    // &str 스트링 리터럴은 언제나 'static을 가짐.
}


// 제네릭 타입, 트레잇, 라이프타임 다 같이 써보기!

fn altogether<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("announcement! {}", ann);

    if x.len() > y.len() {x} else {y}
}