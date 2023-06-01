pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        // assert_eq!(a, b) => a == b 이면 성공 pass, a!=b 이면 panic!
        // assert_ne!(a, b) => a != b 이면 성공, 같으면 panic!
        // assert!(input) => input == true 면 통과, 다르면 panic!
    }
}

// #[cfg(test)] => cargo build 에는 포함시키지 말고, cargo test 에만 포함하려는 의도.

// 비공개 규칙

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod test_private {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));    // 4 == internal_adder(2, 2) ?
    }   
}


