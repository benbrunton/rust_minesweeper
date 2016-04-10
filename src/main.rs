extern crate rand;

mod board;
mod game;

use game::*;
use std::io;
use std::process::exit;

fn main() {
    let game = Game::new();
    
    intro();
    game.display();
    
    while game.in_play() {
        
        show_instructions();
        match handle_input(){
            Command::Exit => break,
            _ => game.display()
        }
    }
    
    exit(end_game());
}

enum Command{
    None,
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
    println!("instructions here");
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

    match &*input {
        "q\n" => Command::Exit,
        _ => Command::None
    }
}
