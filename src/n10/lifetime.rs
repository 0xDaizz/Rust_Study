#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// Lifetime

// 이것 또한 제네릭의 일부. 참조자가 유효한 범위를 정하는 데 중요!

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
