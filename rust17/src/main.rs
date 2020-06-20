
struct Point<T>{
    x : T,
    y : T,
}
impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }

    fn y(&self) -> &T{
        &self.y
    }
}

//Point<T>중 Point<f32>타입만 사용할 수 있음(일부 제너릭 타입만 한정)
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32{
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// Extract the function for delete duplicatino
fn main(){
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_number(&numbers);
    println!("{}", result);

    let chars = vec!['M', 'E', 'S', 'S', 'I'];
    let result = largest_char(&chars);
    println!("{}", result);

    let p = Point{ x : 5, y : 10 };
    println!("p.x : {}, p.y : {}", p.x(), p.y());
}

//List에서 가장 큰 숫자를 찾아내는 function
fn largest_number(list : &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

//slice내에서 가장 큰 char 를 찾음
fn largest_char(list : &[char]) -> char{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}