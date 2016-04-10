extern crate rand;
extern crate regex;

mod board;
mod game;

use game::*;
use std::io;
use std::process::exit;
use regex::Regex;

fn main() {
    let game = Game::new();
    
    intro();
    game.display();
    
    while game.in_play() {
        
        show_instructions();
        let command = handle_input();
        println!("{:?}", command);
        match command{
            Command::Exit => break,
            _ => game.display()
        }
    }
    
    exit(end_game());
}

#[derive(Debug)]
enum Command{
    None,
    Select(String, u8),
    Flag(String, u8),
    Mark(String, u8),
    Unfold(String, u8),
    Exit
}

fn intro(){
    println!("Welcome to Minesweeper by Ben Brunton!");
}


fn end_game() -> i32{
    
    println!("bye!\n");
    return 0;
}

fn show_instructions(){
    // println!("instructions here");
    println!("q :\texit game");
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
    let mark_re = Regex::new(r"^?(\w+)(\d+)\n$").unwrap();
    let unfold_re = Regex::new(r"^#(\w+)(\d+)\n$").unwrap();
    
    match &*input {
        "q\n" => Command::Exit,
        x if uncover_re.is_match(x) => {
            let (row, col) = to_row_col(&uncover_re, x);
            Command::Select(row, col)
        },
        x if flag_re.is_match(x) => {
            let (row, col) = to_row_col(&flag_re, x);
            Command::Flag(row, col)
        },
        x if mark_re.is_match(x) => {
            let (row, col) = to_row_col(&mark_re, x);
            Command::Mark(row, col)
        },
        x if unfold_re.is_match(x) => {
            let (row, col) = to_row_col(&unfold_re, x);
            Command::Unfold(row, col)
        },
        _ => Command::None
    }
}

fn to_row_col(reg:&Regex, input:&str)->(String, u8){
    let opt = reg.captures(input);
    let cap = opt.unwrap();
    let row = cap.at(1).unwrap().to_string();
    let col = cap.at(2).unwrap().parse::<u8>().unwrap();
    
    (row, col)
}
