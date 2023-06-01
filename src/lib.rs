// mod _examples;
pub mod n02;
pub mod n02_upgrade;
pub mod n03;
pub mod n05;
pub mod n08;
pub mod n09;
pub mod n10;
pub mod n11;
pub mod n13;


// 이런 식으로 모듈화가 가능.

// 루트 디렉토리에는 lib.rs를 작성하고 그 속에는 src 내부 폴더들을 작성.

// 각 폴더마다 내부에 mod.rs를 작성하고, mod.rs와 같은 디렉토리에 있는 .rs 파일의 이름 + 폴더의 이름을 모듈로 전부 선언

// 세부 폴더가 있으면 계속 반복