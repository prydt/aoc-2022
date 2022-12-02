use std::io;

fn wins(elf: i32, us: i32) -> i32 {
   if elf == us {
        3 // draw

    // SUPER AMAZING TRICK I LEARNED FROM DANIEL, THANKS :)
    } else if (elf + 1) % 3 == us {
        6 // win
    } else {
        0 // lose
    }
}

fn score(elf: i32, us: i32) -> i32 {
    wins(elf, us) + us + 1
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
            _ => panic!("invalid elf move!")

        };
        chars.next();

        let our_move = chars.next();
        let our_move = match our_move.unwrap() {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!("invalid move for us!")
        };

        total += score(elf_move, our_move);
    }
    println!("our score: {}", total);
}
