use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut max : i32 = 0;
    let mut current : i32 = 0;

    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }

    println!("max calories: {}", max);
}
