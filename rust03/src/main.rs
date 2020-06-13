extern crate rand; //외부에 의존하는 크레이트가 있음을 알림
use std::io; //입출력 library
use std::cmp::Ordering; //Ordering타입 3가지 (Less, Greate, Equal)
use rand::Rng; //정수 생성기가 구현한 method들을 정의한 trait

fn main(){
    println!("Guess the number!");

    //1부터 101 사이의 임의의 숫자를 생성하는 random함수
    //thread_rng -> OS가 seed를 정하고 현재 thread에서만 사용되는 정수 생성기
    //gen_range -> 범위 지정
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");
        //변수 선언(mut -> 가변 변수 선언)
        let mut guess = String::new(); //String 타입 변수 선언
    
        //guess에 입력하기 &mut -> 참조자(data를 여러번 memory로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자)
        //.expect -> 잠재된 실패에 대한 처리(data type 등)
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        //변수를 u32(unsigned 32bit Integer)로 casting
        //trim -> String의 first, last 빈칸 제거(string input시 \n가 입력되므로 제거)
        //parse -> 문자열을 숫자형으로 parsing함
        let guess:u32 = match guess.trim().parse(){
            //parse가 성공적으로 수행된 경우 결과값을 가진 OK를 반환
            //parse 실패한 경우 Err 반환
            Ok(num) => num,
            Err(_) => {
                println!("please input unsigned int input");
                continue;
            }
        };
        println!("You guessed: {}", guess);
    
        //입력한 숫자 비교
        //cmp : 두 값을 비교하여 비교 가능한 모든 함수 호출 가능
        //match : 표현식에 주어진 값이 패턴과 맞다면 실행
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yow win!");
                break;
            }
        }
    }
}