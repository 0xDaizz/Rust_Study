fn main() {

// 1.
let user1 = Person {
    name: String::from("me"),
    age: 10
};

println!("#1. name/age, {:?}", user1.name_and_age());

// 2.
option_example(5, 4);

option_example(3, 0);

// 3.
vec_collection();

// 4.


}

// 1. 구조체 & 메서드
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn name_and_age(&self) {
        println!("{} and {}", self.name, self.age)
    }
    
}

// 2. Option  열거형
fn option_example(a: i32, b: i32) {
    match a.checked_div(b) {
        Some(answer) => println!("answer is {}", answer),
        None => println!("Cannot divide into 0."),
    }
}
// 시발 어떻게 써먹는거야.

// 3. Vec 컬렉션
fn vec_collection() {

    let mut list = vec![1, 2, 3, 4, 5];

    for i in &mut list {
        *i *= 2;
    } 

    println!("{:?}", list);
}

// 4. Error handling
fn error_handle() {
    let word = "asdfasdfasdf";

    let parsed_word = word.parse()::i32
}