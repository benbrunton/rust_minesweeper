extern crate rand;
extern crate regex;

mod board;
mod game;

use game::*;
use std::io::{self, Write};
use std::process::exit;
use regex::Regex;

fn main() {
    let mut game = Game::new();
    
    intro();
    game.display();
    
    while game.in_play() {
        
        show_instructions();
        let command = handle_input();
        match command{
            Command::Exit => break,
            Command::Select(col, row) => game.select(col, row),
            Command::Mark(col, row) => game.mark(col,row),
            Command::Flag(col, row) => game.flag(col,row),
            Command::Unfold(col, row) => game.unfold(col, row),
            _ => ()
        }
        
        game.display();
    }
    
    exit(end_game());
}

#[derive(Debug)]
enum Command{
    None,
    Select(String, u32),
    Flag(String, u32),
    Mark(String, u32),
    Unfold(String, u32),
    Exit
}

fn intro(){
    println!("Welcome to Minesweeper by Ben Brunton!");
}


fn end_game() -> i32{
    
    println!("game over!");
    println!("bye!\n");
    return 0;
}

fn show_instructions(){
    println!("find all mines to win!");
    println!("q :\texit game");
    println!("a0 :\tselect square a0");
    println!("!a0 :\tflag a0 as a mine");
    println!("?a0 :\tmark a0 with a ?");
    println!("#a0 :\tuncover unmarked neighbours of a0");
    print!("your move: ");
    io::stdout().flush().unwrap(); // make sure "your move" is output to stdout
}

fn handle_input() -> Command{
    
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => create_move(input),
        Err(error) => {println!("error: {:?}", error); Command::None}
    }
}

fn create_move(input:String) -> Command{
    
    let uncover_re = Regex::new(r"^(\w+)(\d+)\n$").unwrap();
    let flag_re = Regex::new(r"^!(\w+)(\d+)\n$").unwrap();
    let mark_re = Regex::new(r"^\?(\w+)(\d+)\n$").unwrap();
    let unfold_re = Regex::new(r"^\#(\w+)(\d+)\n$").unwrap();
    
    match &*input {
        "q\n" => Command::Exit,
        x if uncover_re.is_match(x) => {
            let (col, row) = to_row_col(&uncover_re, x);
            Command::Select(col, row)
        },
        x if flag_re.is_match(x) => {
            let (col, row) = to_row_col(&flag_re, x);
            Command::Flag(col, row)
        },
        x if mark_re.is_match(x) => {
            let (col, row) = to_row_col(&mark_re, x);
            Command::Mark(col, row)
        },
        x if unfold_re.is_match(x) => {
            let (col, row) = to_row_col(&unfold_re, x);
            Command::Unfold(col, row)
        },
        _ => Command::None
    }
}

fn to_row_col(reg:&Regex, input:&str)->(String, u32){
    let opt = reg.captures(input);
    let cap = opt.unwrap();
    let col = cap.at(1).unwrap().to_string();
    let row = cap.at(2).unwrap().parse::<u32>().unwrap();
    
    (col, row)
}
