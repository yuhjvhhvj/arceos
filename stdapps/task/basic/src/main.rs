use std::thread;

fn main() {
    let computation = thread::spawn(|| {
        // Some expensive computation.
        42
    });

    println!("Thread join...");
    let result = computation.join().unwrap();
    println!("Thread: basic ok! Result: [{result}].");
}
