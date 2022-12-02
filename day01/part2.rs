use std::io;
use std::collections::BinaryHeap;

fn main() {
    let lines = io::stdin().lines();

    let mut current : i32 = 0;

    let mut heap = BinaryHeap::new();

    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            heap.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    // let total: i32 = heap.into_iter_sorted().take(3).sum();
    let total: i32 = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();

    println!("total of top 3 calories: {}", total);
}
