mod board;

use board::*;

fn main() {
    let board = Field::new(10, 10);
    println!("{}", board);
}
