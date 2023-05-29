// 튜플과 배열 특집

// 우선 : 튜플, 배열 모두 길이는 불변!!

fn main() {
    
    // 1. 튜플

    // tuple은 타입이 다른 것들끼리도 묶을 수 있음!
    let tup1: (i32, bool, &str) = (52, true, "hi");
        // 여기서 : "hi"는 String이 아니라 &str 타입이 되어야 함.
        // String은 변경될 여지가 있는 문자열에 주로 쓰고, String::from("asdf"); 이런 식으로 혹은 "asdf".to_string()으로 사용함
        // 일반적인 문자열은 &str 꼴로 사용

    // 이렇게 일대일 대응의 형태로 튜플 안의 요소를 변수에 대입할 수 있음
    let (x, y, z) = tup1;

    // 결과확인
    println!(" x = {}, y = {}, z = {}", x, y, z);



    // 2. 배열

    // 배열의 경우는 조금 다르다.
    // 모든 요소가 타입이 같아야 함.

    let a1 = [1, 2, 3, 4, 5];   // 이게 기본적인 배열 생성

    let mut a2 = [50; 5]; // [요소; 반복 횟수] 꼴로 구성할 수도 있음

    a2[3] = 0;  // 배열의 요소를 이렇게 바꿀 수 있음 (단, 해당 배열이 mut 선언 되어있어야 함)

    // println!("a1: {}", a1);     // 일반적인 출력. 배열은 std::fmt::Display 가 구현되지 않았기 때문에 불가능.


    // 만약 배열을 일반적인 포맷으로 출력하고 싶다면, 이렇게 배열의 요소를 끌어내야 함.
    for i in 0..a1.len() {
        println!("{}", a1[i]);
    }


    println!("a2: {:?}", a2);   // 디버깅 출력. 배열을 [1, 2, 3, 4, 5] 이런 식으로 출력하고 싶다면 디버깅 출력을 해야 함 ({:?})



    
}