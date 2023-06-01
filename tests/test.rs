// for integration test!!

// tests 폴더는 특별 취급되어, #[cfg(test)]를 쓰지 않아도 됨.

use rust_study::*;  // 메인 프로젝트의 모든 모듈을 불러옴.

// 다른 파일로 가져오는 것이므로, 가져오고 싶은 모듈은 반드시 pub 처리되어있어야 함.

// 예를 들어, src/n11/test_study.rs의 add_two 함수를 가져오겠음.

#[test]
fn test_add_two() {
    assert_eq!(4, n11::test_study::add_two(2));  
    // 풀네임 : rust_study::n11::test_study::add_two()
    // 맨 위 use 구문에서 이어 쓴다고 생각하면 ok
}


use rust_study::n10::*;

#[test]
fn modified_largest_test() {
    let numbers = vec![12, 43, 55, 48, 29, 19];

    assert!(trait_study::modified_largest(&numbers) == &55);
}

// 결과


// running 2 tests
// test n11::test_study::tests::it_works ... ok
// test n11::test_study::test_private::internal ... ok

// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running unittests src/main.rs (target/debug/deps/rust_study-8308769e0d83a70f)

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running tests/test.rs (target/debug/deps/test-2c059aff30e0ac9d)

// running 2 tests
// test test_add_two ... ok
// test modified_largest_test ... ok

// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests rust_study

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s