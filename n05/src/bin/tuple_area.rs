// main.rs의 사각형 넓이 구하는 함수를, 튜플로 리팩터링할 것임.

fn main() {

    let square = (60, 60);  // 튜플로 가로세로 정의

    println!("The area is {}.", area(square));

}


fn area(length: (u32, u32)) -> u32 {    // length라는 파라미터는 : u32, u32 타입의 튜플이다. 그리고 u32를 돌려준다.
    length.0 * length.1 // length 튜플의 0번째 자리 값과 1번쨰 자리 값을 곱한다.
}