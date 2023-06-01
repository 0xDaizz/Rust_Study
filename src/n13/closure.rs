// Closure 함수
fn how_to_closure() {
    fn  _add_one_v1   (x: u32) -> u32 { x + 1 }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };


// 타입을 표기하지 않은 경우
// let add_one_v3 = |x| x + 1;
// 이 경우, 처음 클로저가 호출될 때의 타입을 확정함.

// 위의 let _add_one_v2를 보면, Typescript의 화살표 함수와 굉장히 유사하다는 것을 알 수 있다.

    let str_length = |x: String| -> usize {x.len()};

    // Typescript
    // const c = (x:string): number => (x.length);

    let a = str_length(String::from("Hello"));   // 클로저로 정의한 함수 사용하기 
    println!("{a}");
}


// 클로저는 3개의 Trait 중 하나를 가진다.

// Fn, FnMut, FnOnce

// 클로저가 둘러싼 스코프 내의 변수를 사용하며, 주변 환경을 캡쳐 함.

fn closure_trait() {

    let x = 5;
    let y = "Hi";

    let _a= |w: i32| w == x;

    let _b = move |v: &str| v == y;
    // 이것은 v가 &str이기 때문에(Copy 불가능), move를 사용해 y의 소유권을 넘겨받음.
    // 이제 원래의 y는 사라짐.

    println!("{}", _b("hi"));   // false

}


pub fn main() {
    how_to_closure();

    closure_trait();
}