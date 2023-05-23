fn main() {
    let length1 = 75;
    let length2 = 45;

    println!("the area is {}.", area(length1, length2));
}

fn area (x: u32, y: u32) -> u32 {
    x * y
}

