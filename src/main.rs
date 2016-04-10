extern crate rand;

mod board;

use board::*;

fn main() {
    let board = FieldView::new(10, 10, 30);
    println!("{}", board);
    
    board.showResult();
}
