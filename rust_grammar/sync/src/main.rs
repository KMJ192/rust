use std::thread;
use std::sync::mpsc;

use std::time::Duration;

const NUM_TIMERS: usize = 24;

fn timer(d : usize, tx: mpsc::Sender<usize>){
    thread::spawn(move ||{
        println!("{}: Setting timer...", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("{} : sent!", d);
        tx.send(d).unwrap();
    });
}

fn main() {
    let mut c = vec![];
    
    for i in 0..10{
        c.push(thread::spawn(move || {
            println!("thread number {}", i);
        }));
    }
    for i in 0..10{
        println!("main tread");
    }

    println!();

    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("vector : {:?}", v);
    });

    handle.join();

    //mpsc::channel() fifo형태 대기열 통신 primitives
    let (tx, rx) = mpsc::channel();
    //tx에서 rx로 data를 send
    thread::spawn(move || {
        tx.send(42).unwrap(); 
    });

    //rx에서 data를 receive
    println!("{}", rx.recv().unwrap());

    println!();
    
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_TIMERS{
        timer(i, tx.clone());
    }

    for v in rx.iter().take(NUM_TIMERS){
        println!("{} : receiverd", v);
    }
}
