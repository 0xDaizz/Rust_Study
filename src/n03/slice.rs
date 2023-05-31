fn main() {

    let s = String::from("Hello World");

    // 스트링을 잘라서 가져옴
    let s1 = &s[0..5];  // = &s[..5]와 동일. // s1 = "Hello"

    let s2 = &s[6..11]; // = &s[6..]와 동일. //s2 = "World"

    println!("{}\n{}", s1, s2);


    let fw = first_word(&s);

    println!("first_word of s is {}.", fw);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}