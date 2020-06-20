use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Green"), 20);
    scores.insert(String::from("Blue"), 30);

    //HashMap 요소 Data 갱신
    //HashMap의 요소 중 Key가 Blue인 Value를 변경할 수 있음
    scores.insert(String::from("Blue"), 40); 

    //HashMap의 요소 접근
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
    println!("{:?}", scores);

    let text = "Lionel Andres\nMessi Cuccitini Messi";
    let mut map = HashMap::new();
    //split_whitespace() => 공백값을 기준으로 잘라냄
    for word in text.split_whitespace(){
        // map.entry(word).or_insert(0)
        //map에 해당 단어가 키로 존재하는지 체크(entry)하여 or_insert(0)호출
        //or_insert(0) -> 값이 이미 있는 값을 return, 없다면 0의 값을 map에 insert
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
