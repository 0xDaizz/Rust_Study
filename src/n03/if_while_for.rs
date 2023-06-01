// While 사용 예시
#![allow(dead_code)]


fn main() {
    let mut count = 3;

    while count > 0 {   //:, ; 등등 아무것도 넣지 않음.
        println!("{}", count);

        count -= 1;
    }

    println!("LIFTOFF!!");



    // while 이용한 컬렉션 반복
    // 이 방법은 느리고 비추천하는 방법
    // for를 사용하는 게 나음

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("value = {}", a[index]);

        index += 1;
    }
    // 이렇게 하면, 1. 배열의 길이와 반복 횟수가 안 맞는 경우 패닉 오류를 뿜기 쉽다. 이걸 신경써줘야 함.
    // 2. 반복문을 통해 반복할 때마다 컴파일러가 조건 검사를 계속 함. 시간이 오래 걸리게 됨.


    // 따라서 위 코드는 for를 이용해 간단히 해야 함.
    // 1번 방법
    for element in a.iter() {
        println!("value = {}", element);
        // iter() 는 배열의 첫 요소부터 차례로 할당
    }


    // 2번 방법
    for i in 0..a.len() {
        println!("value = {}", a[i]);

        // 0..a.len() => 0부터 a배열의 길이까지!
    }

    // iter() 의 반대로 역순으로 가져오는 것은 rev()
    for i in (0..a.len()).rev() {
        println!("reverse_value: {}", a[i]);
    }


}