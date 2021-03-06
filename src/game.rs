
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
    }
    
    pub fn select(&mut self, col:String, row:u32){
        self.board.select(col, row);
    }
    
    pub fn mark(&mut self, col:String, row:u32){
        self.board.mark(col, row);
    }
    
    pub fn flag(&mut self, col:String, row:u32){
        self.board.flag(col, row);
    }
    
    pub fn unfold(&mut self, col:String, row:u32){
        self.board.unfold(col, row);
    }
    
    pub fn in_play(&self) -> bool {
        self.board.in_play()
    }
}