use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    // Commandline 인자를 벡터로 모으고, 출력
    let args: Vec<String> = env::args().collect();  
    println!("{:?}", args);

    // query와 filename을 보관하는 두 변수 정의
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {} \nIn file {}", query, filename);

    // 여기서 cargo run test sample.txt 로 인자 test와 smaple.txt를 줄 수 있다.


    let mut f = File::open("poem.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong");

    println!("With text:\n{}", contents);

    // 여기까지 해서, poem.txt를 읽어 왔다.


    


}