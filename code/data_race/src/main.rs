use std::thread;
use std::time::Duration;

use std::sync::{Arc, Mutex};

const THREADS: usize = 10;
const COUNT: usize = 50;

pub fn ex01() {
    let mut x = 1;
    let mut handles = vec![];
    for t in 0..THREADS {
        handles.push(thread::spawn(move || {
            for _ in 0..COUNT {
                let a = x + 1;
                thread::sleep(Duration::from_micros(1));
                println!("Thread {} wrote {}", t, a);
                x = a;
            }
        }))
    }
    for handle in handles {
        handle.join().expect("Thread shouldn’t have panicked");
    }
    if x != THREADS * COUNT + 1 {
        println!("Got x = {}", x);
        println!("Expected to get {}", THREADS * COUNT + 1);
    } else {
        println!("Did not reproduce the issue.");
    }
}

pub fn ex02() {
    let x = Arc::new(Mutex::new(1));

    let mut handles = vec![];
    for t in 0..THREADS {
        let x_clone = Arc::clone(&x);
        handles.push(thread::spawn(move || {
            
            for _ in 0..COUNT {
                let mut data = x_clone.lock().unwrap();

                let a = *data + 1;
                thread::sleep(Duration::from_micros(1));
                println!("Thread {} wrote {}", t, a);
                *data = a;
            }
        }))
    }
    for handle in handles {
        handle.join().expect("Thread shouldn’t have panicked");
    }
    
    let data = x.lock().unwrap();
    if *data != THREADS * COUNT + 1 {
        println!("Got x = {}", *data);
        println!("Expected to get {}", THREADS * COUNT + 1);
    } else {
        println!("Did not reproduce the issue.");
    }        
}

fn main() {
    // ex01();
    ex02();    
}
