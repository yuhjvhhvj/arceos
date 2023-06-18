extern crate alloc;

use alloc::sync::Arc;
use alloc::vec::Vec;
use std::time::Duration;
use std::sync::Barrier;
use std::thread;

const NUM_DATA: usize = 2_000_000;
const NUM_TASKS: usize = 16;

fn sqrt(n: &u64) -> u64 {
    let mut x = *n;
    loop {
        if x * x <= *n && (x + 1) * (x + 1) > *n {
            return x;
        }
        x = (x + *n / x) / 2;
    }
}

fn main() {
    extern "Rust" {
        fn sys_rand_u32() -> u32;
    }

    let vec = Arc::new(
        (0..NUM_DATA)
            .map(|_| unsafe { sys_rand_u32() } as u64)
            .collect::<Vec<_>>(),
    );
    let expect: u64 = vec.iter().map(sqrt).sum();

    // equals to sleep(500ms)
    thread::sleep(Duration::from_millis(500));

    let barrier = Arc::new(Barrier::new(NUM_TASKS));

    let mut tasks = Vec::with_capacity(NUM_TASKS);
    for i in 0..NUM_TASKS {
        let b = Arc::clone(&barrier);
        let vec = vec.clone();
        tasks.push(thread::spawn(move || {
            let left = i * (NUM_DATA / NUM_TASKS);
            let right = (left + (NUM_DATA / NUM_TASKS)).min(NUM_DATA);
            println!(
                "part {}: {:?} [{}, {})",
                i,
                thread::current().id(),
                left,
                right
            );

            let partial_sum: u64 = vec[left..right].iter().map(sqrt).sum();
            b.wait();

            println!("part {}: {:?} finished", i, thread::current().id());
            partial_sum
        }));
    }

    let actual = tasks.into_iter().map(|t| t.join().unwrap()).sum();
    println!("sum = {}", actual);
    assert_eq!(expect, actual);

    println!("Parallel summation tests run OK!");
}
