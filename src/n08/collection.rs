#![allow(dead_code)]

// 1. 첫 번째 컬렉션 : Vector

fn main() {
    vector_study();

    string_study();

    hashmap_study();
}

fn vector_study() {


    let _v: Vec<i32> = Vec::new();  // 새로운 벡터 생성. Vec<i32> 타입의 개체를 new() 생성

    // or, We can generate vector by this macro : vec!

    let mut vv = vec![1, 2, 3];

    // push(): 끝부분 추가, pop(): 끝부분 삭제
    vv.push(4);

    println!("{:?}", vv);


    // 벡터의 요소 읽어내기 2가지

    let _second: &i32 = &vv[1];  // 편리하지만 만약 해당 항목이 없으면 Panic!

    let __second: Option<&i32> = vv.get(1);   // Option 타입을 반환함. Some<&i32> 혹은 None 반환.

    // 만약 참조 안 하고 벡터를 가져오면..?

    let mut vvv = vec![10, 20, 30, 40];

    let _first: i32 = vvv[0];

    println!("vvv modified : {:?}", vvv); 
    // vvv[0]의 타입이 i32이라, Copy trait가 있어서 복사 가능. 
    // 하지만 String 같은 타입을 이렇게 하면 오류 발생.
    // 따라서 &참조를 해야 함.


    // 반복처리 w/ for loop

    for i in &mut vvv {
        *i += 100;  // i는 vvv에서 가져온 가변 참조 &mut vvv[0]. 가져온 참조를 바탕으로 원래 값을 바꿔야 하니, 역참조 *i 를 써야 함.
    }
    println!("{:?}", vvv);


    // Enum을 이용한 다른 타입으로 구성된 벡터 만들기

    enum Everything {
        Integer(i32),
        String(String),
        Boolean(bool),
    }

    let _every_vec = vec![
        Everything::Integer(22),
        Everything::String(String::from("HI")),
        Everything::Boolean(true)
    ];

    // 만약 여러 타입을 벡터에 넣고 싶은데 모든 값의 타입을 다 알지 못한다면, 이런 방식으로는 불가능
    // Trait object를 이용해야 함.(차후 배움)
}    


// 2. 두 번째 컬렉션 : String

fn string_study() {

    // new Empty String
    let _a = String::new();

    // string을 만드는 두 가지 방법 -- "hi" 는 스트링 슬라이스, 스트링 리터럴. str
    let b = ("hello").to_string();
    let c = String::from("hihi");

    // str과는 달리, String은 수정 가능하다. 따라서: 
    
    // push_str, push -- push_str는 문자열, push는 한 글자 가능!
    let mut d = ("I like ").to_string();
    d.push_str("apple.");

    println!("{}", d);


    // + 를 이용해 문자열 더하기

    let bc = b + &c;    // add 함수를 가져온 것이기에, 첫 문자열은 그대로 소유권까지 가져오고, 두 번째 이후는 참조 &를 가져옴.
    // b is not vaild anymore!!!
    println!("{}", bc);

    // 불편하기 때문에, format! 매크로를 사용하면 됨!!!
    // everything is still valid!!
    let attached_strings = format!("{} - {}", bc, d);
    println!("{}", attached_strings);


    // 스트링을 슬라이싱, 인덱싱하는 것은 복잡한 일이며, 때때로 좋은 아이디어가 아니다..
}


// 3. 세 번째 컬렉션 : HashMap

use std::collections::HashMap;  // 해쉬맵은 불러와야 한다!!

fn hashmap_study() {

    // HashMap<K, V> => K 키와 V 값을 묶어 저장. 딕셔너리와 유사하다.
    // 모든 K 키와 V 값은 각각 끼리끼리 같은 타입이어야 함!!

    let mut prices = HashMap::new();

    prices.insert(String::from("Apple"), 5000);
    prices.insert(String::from("Orange"), 4000);

    println!("{:?}", prices);


    // Tuple의 벡터에 대해 Collect 메서드를 사용해 해쉬맵을 만들 수도 있다!

    let teams = vec!["Kia", "LG", "Lotte", "Samsung"];
    let final_scores = vec![30, 50, 60, 80];

    let all_scores: HashMap<_,_> = teams.iter().zip(final_scores.iter()).collect();
    // 쫌 복잡하긴 하네..

    println!("{:?}", all_scores);


    // 해쉬맵 내의 값 접근하기

    let apple_price = prices.get(&String::from("Apple"));   
    // 가져올 때는 & 꼭꼭!! 그나저나 반환값이 Option<&i32>네... get()의 특징!

    println!("{}", apple_price.unwrap_or(&0));    // 반환이 Some(&i32) 아니면 None이라서, 처리해 줘야 함!


    // for loop 이용하기 -- Random 순서로 출력!
    for (key, value) in &all_scores {
        println!("{}: {}", key, value);
    }


    // 해쉬맵의 값 갱신

    // 덮어쓰기 = insert

    prices.insert(String::from("Apple"), 6000);  // 사과 값이 1000원 더 올랐음

    println!("{:?}", prices);


    // 키에 할당된 값이 없을 경우에만 삽입하기 = entry

    prices.entry(String::from("Apple")).or_insert(7000);  // Key가 원래 있으니까 기각
    prices.entry(String::from("Banana")).or_insert(3000); // Key가 없던 값이니까 추가!
    // or_insert() : 이미 있을 경우 Entry 키에 대한 값을 반환. 없으면 파라미터를 값으로 해서 수정된 entry를 반환.

    println!("{:?}", prices);


    // entry를 사용하는 좋은 예시

    // Key에 대한 값을 찾아서, 예전 값에 기초해 값을 갱신하는 것 (ex : +=)!

    // 예제로, 주어진 문장에서 각 단어가 몇 번 나왔는지 세는 것!

    let text = "I love you I like you";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


    // HashMap은 해쉬함수를 사용. 빠르진 않지만 보안은 좋다.
    // 빠른 걸 원한다면 Buildhasher 트레잇을 구현한 다른 hasher를 찾아보면 좋을 듯.ㄴ
}