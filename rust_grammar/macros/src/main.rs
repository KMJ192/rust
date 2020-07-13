macro_rules! a_macro {
    () => {
        println!("Run Macro");
    };
}

macro_rules! x_and_y {
    (x => $e:expr) => (println!("X : {}", $e));
    (y => $e:expr) => (println!("Y : {}", $e));
}

macro_rules! build_fn{
    ($func_name:ident) => {
        fn $func_name(){
            println!("{:?}", stringify!($func_name));
        }
    }
}

macro_rules! print_ex{
    ($e:expr) => (
        println!("{:?} = {:?}", stringify!($e), $e);
    )
}

macro_rules! exam{
    ($l:expr; and $r:expr) => (
        println!("{:?} and {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l && $r
        )
    );

    ($l:expr; or $r:expr) =>(
        println!("{:?} or {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l || $r
        )
    );
}

fn main() {
    a_macro!();

    x_and_y!(x => 10);
    x_and_y!(y => 10 + 20);

    build_fn!(test_function);
    test_function();

    print_ex!({
        let y = 20;
        let z = 30;
        z + y + 10 * 3 * 100
    });

    exam!(1 == 1; and 2 == 1 + 2);
}

