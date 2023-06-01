#![allow(dead_code)]
#![allow(unused_variables)]

// trait 배우기 전에, struct 복습

struct ThisIsAStruct {
    one: String,
    two: i32,
    three: bool,
}

// 인스턴스 생성
fn _instance() {
    let instance_one = ThisIsAStruct {
        one: String::from("ONE"),
        two: 2,
        three: true,
    };
}

// 인스턴스의 반환
fn return_instance(one: String, two: i32, three: bool) -> ThisIsAStruct {
    ThisIsAStruct {
        one,    // one = one, 과 동일!
        two,
        three,
    }
}

// Tuple 구조체
fn tuple_struct() {
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
}


// struct 안에 method 정의하기 
impl ThisIsAStruct {
    fn print(&self) {
        println!("{}, {}, {}", &self.one , &self.two, &self.three);
    }
}

fn struct_print() {
    let instance_a = ThisIsAStruct { one: String::from("hi"), two: 1010, three: false };

    instance_a.print();
}


// Trait : 여러 구조체들의 공통된 양식을 규정하는 것.
// JS, TS의 interface와 비슷.

pub trait Hello {
    fn hi(&self) -> String;
}

pub struct Person {
    pub name: String,
    pub age: i32, 
}

impl Hello for Person {
    fn hi(&self) -> String {
        format!("{}, {}", &self.name, &self.age)
    }
}

// 제네릭 도입하기!
// PartialOrd: > < >= <= 연산을 수행할 수 있는 타입만으로 한정!
// Copy를 같이 구현해야 함!

fn largest<T: PartialOrd + Copy>(arr: &[T]) -> T {
    let mut largest = arr[0];
    for &item in arr {
        if  item > largest {
            largest = item;
        }
    }
    largest
}

// list[0]을 largest로 옮기는 동작이 아닌, 참조를 하는 방식으로 한다면 
// Copy trait를 구현할 필요도 없어짐!

fn modified_largest<T: PartialOrd>(arr: &[T]) -> &T {
    let mut largest = &arr[0];  
    // return Type을 &T로 참조로 수정하고, 
    // return할 값인 largest를 정의할 때 arr[0]의 참조를 가져옴!
    
    for item in arr {
        if item > largest {
            largest = item;
        }
    }
    largest
}




fn main() {

    struct_print();


}