//모듈의 스코프 내에 정의된 코드를 다른 위치에서 찾기
pub mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
