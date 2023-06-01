// 반복자.

// Iterator trait을 가짐.
// iterator: next 메서드를 가짐.

// iter() : 벡터나 배열에 대한 반복자 생성
// map() : 반복자의 각 요소에 () 속의 함수를 적용
// collect() : 반복자를 소비해서 결과를 벡터로 모음.

// 반복자는 게으르기 때문에, collect() 등으로 일부러 소모해주지 않으면 아무 작업도 안 함..

pub fn main() {

    let numbers = vec![1, 2, 3, 4, 5, 6, 7];

    let num_squared: Vec<i32> = numbers.iter().map(|&x: &i32| x.pow(3)).collect();
    // pow() 는 참조에 대해서 동작 안 하니, |&x: &i32|를 통해 x를 원래 정수 i32로 바인딩해야 함.

    println!("{:?}", num_squared);
}