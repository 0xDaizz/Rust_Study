#![allow(dead_code)]
#![allow(unused_variables)]
// Error management

// Result 타입

// enum Result<T, E> {
//     Ok(T),  // 성공하면 Ok
//     Err(E), // 실패하면 Err
// }


// Match를 이용해 Result를 처리하는 법

use std::fs::File;
use std::io::ErrorKind;

fn file_creation() {

    let f = File::open("hi.txt");

    let f = match f {   // 변수 Shadowing
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {   // match guard if절을 만족하면 그걸 넘겨받는 ref error의 Err를 호출.
            match File::create("./src/n09/hello.txt") {
                Ok(fc) => fc,   // File creation success
                Err(e) => {
                    panic!("Tried to generate the file but failed: {:?}", e)
                },
            }
        },
        Err(error) => { // match guard가 거짓이면 여기의 Err를 호출.
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}


// unwrap, expect

fn unwrap_expect_study() {
    let f = File::open("./src/n09/hello.txt").unwrap();
    // 만약 파일을 열기에 실패하는 경우 panic!

    let f = File::open("./src/n09/hello.txt").expect("Failed to open hello.txt");
    // expect 또한 실패하면 panic! 하지만, 오류 메시지를 지정해 보낼 수 있음!!

    // 여러 개의 unwrap이 있는 경우, 디버깅 하면서 어느 unwrap이 문제인지 찾기 힘든 데 반해, 
    // expect를 쓰면 쉽게 알아낼 수 있다!
}


// 에러 전파

use std::io;
use std::io::Read;

fn error_spreading() -> Result<String, io::Error>{
    let f = File::open("./src/n09/hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}

// 물음표 연산자 사용하기

// ? -> result를 반환하는 메서드를 실행했을 때, 
// 반환한 Result가 Ok(T)라면 => T를 반환하고 계속.
// 만약 Err(e) 면 Err(e) 를 그대로 반환

fn _using_question() -> Result<String, io::Error>{

    // 위의 함수와 똑같은 기능

    let mut f = File::open("./src/n09/hello.txt") ? ;   // 물음표 등장 : File을 성공적으로 열었으면 let f = File을 할당.

    let mut s = String::new();

    f.read_to_string(&mut s) ? ;    // 또 등장 : f의 내용을 잘 읽어서 s에 저장했으면 Ok(s)를 반환하고 계속. 아니면 Err.

    Ok(s)

}


// 더 줄일 수도 있음!!

fn _shorten_question() -> Result<String, io::Error>{

    let mut s = String::new();

    File::open("./src/n09/hello.txt")?.read_to_string(&mut s)?;

    Ok(s)

}




fn main() {
    file_creation();

    unwrap_expect_study();

    let result = error_spreading();
}