
use board::*;

pub struct Game{
    board: FieldView
}

impl Game {
    pub fn new() -> Game {
        Game{ board: FieldView::new(10, 10, 30) }
    }
    
    pub fn display(&self){
        println!("{}", self.board);
        
        self.board.show_result();
    }
    
    pub fn in_play(&self) -> bool {
        true
    }
}