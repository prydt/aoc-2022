use std::io;

// #[derive(Copy, Clone)]
// enum Moves {
//     Rock = 1,
//     Paper,
//     Scissors
// }
//
//
//
// 0
// 1
// 2

// #[derive(Copy, Clone)]
// enum Outcome {
//     Lose = 0,
//     Draw = 3,
//     Win = 6
// }

fn wins(elf: i32, us: i32) -> i32 {
    // match elf {
    //     Moves::Rock => match us {
    //         Moves::Rock => Outcome::Draw,
    //         Moves::Paper => Outcome::Win,
    //         Moves::Scissors => Outcome::Lose,
    //     },
    //     Moves::Paper => match us {
    //         Moves::Rock => Outcome::Lose,
    //         Moves::Paper => Outcome::Draw,
    //         Moves::Scissors => Outcome::Win,
    //     },
    //     Moves::Scissors => match us {
    //         Moves::Rock => Outcome::Win,
    //         Moves::Paper => Outcome::Lose,
    //         Moves::Scissors => Outcome::Draw,
    //     }
    // }
    if elf == us {
        3 // draw
    } else if (elf + 1) % 3 == us {
        6 // win
    } else {
        0 // lose
    }
}

fn score(elf: i32, us: i32) -> i32 {
    // (wins(elf, us) as i32) + (us as i32)
    wins(elf, us) + us + 1
}

fn main() {

    let lines = io::stdin().lines();
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let mut chars = line.chars();

        /*
         * A - rock
         * B - paper
         * C - scissors
         */
        let elf_move = chars.next();
        let elf_move = match elf_move.unwrap() {
            // 'A' => Moves::Rock,
            // 'B' => Moves::Paper,
            // 'C' => Moves::Scissors,
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("invalid elf move!")

        };
        chars.next();

        /*
         * X - rock
         * Y - paper
         * Z - scissors
         */
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
