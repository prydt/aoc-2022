use std::io;

fn move_score(elf: i32, outcome: i32) -> i32 {
    match outcome {
        3 => elf,
        0 => (elf + 2) % 3, // THIS TOOK TOO LONG
        6 => (elf + 1) % 3,
        _ => panic!("invalid outcome!"),
    }
}

fn main() {
    let lines = io::stdin().lines();
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let mut chars = line.chars();

        // this whole conversion can be a subtraction
        let elf_move = chars.next();
        let elf_move = match elf_move.unwrap() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("invalid elf move!"),
        };
        chars.next();

        let outcome = chars.next();
        let outcome = match outcome.unwrap() {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("invalid move for us!"),
        };

        total += outcome + move_score(elf_move, outcome) + 1;
        println!("{} + {}", outcome, move_score(elf_move,outcome) + 1);
    }
    println!("our score: {}", total);
}
