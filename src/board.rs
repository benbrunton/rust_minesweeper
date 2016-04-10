use std::fmt;
use rand;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Flag,
    Mark,
    Closed,
    Open
}

impl fmt::Display for Tile{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", "█")
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TileState {
    Mine,
    Number(u32)
}

impl fmt::Display for TileState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = match self {
            &TileState::Mine        => "♦".to_string(),
            &TileState::Number(n)   => n.to_string()
        };
        write!(f, "{} ", icon)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Field(Vec<Vec<TileState>>);

impl Field {
    pub fn new(width:u32, height:u32, mines:usize) -> Field {

        let mut rng = rand::thread_rng();
        let width_range = (0..width).collect::<Vec<u32>>();
        let height_range = (0..height).collect::<Vec<u32>>();
        
        // get mine positions
        let mine_positions = (0..mines).map(|_| 
                (
                    rng.choose(&width_range).unwrap().clone(), 
                    rng.choose(&height_range).unwrap().clone())
                ).collect::<Vec<(u32, u32)>>();


        // create complete board
        let mut tiles = vec!();
        for y in 0 .. height {
            let mut row = vec!();
            for x in 0 .. width {
                let tile = if is_mine(&mine_positions, x, y) {
                    TileState::Mine
                }else{
                    TileState::Number(count_mines(&mine_positions, x, y))
                };
                
                row.push(tile);
            }
            
            tiles.push(row);
        }
        
        Field(tiles)
        
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let _ = write!(f, " ");
        let letters = vec!('a', 'b', 'c', 
            'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 
            'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
            
        let mut cols = letters.iter();

        let _ = write!(f, " ");
        for _ in 0..self.0.iter().count() {
            let _ = write!(f, "{} ", cols.next().unwrap());
        }
        let _ = write!(f, "\n");
        
        let mut row_num = 0;
        for row in self.0.iter(){
            let _ = write!(f, "{} ", row_num);
            for tile in row.iter(){
                let _ = write!(f, "{}", tile);
            }
            let _ = write!(f, "\n");
            row_num = row_num + 1;
        }
        writeln!(f, "\n")
        
    }
}

fn is_mine(positions: &Vec<(u32, u32)>, x: u32, y: u32) -> bool {
    positions.iter().any(|&(xpos, ypos)| xpos == x && ypos == y)
}

fn count_mines(positions: &Vec<(u32, u32)>, x: u32, y: u32) -> u32 {
    let mut surrounding:Vec<(u32, u32)> = vec!();
    
    if x > 0 {
        surrounding.push((x - 1, y));
        surrounding.push((x - 1, y + 1));
    }
    
    if y > 0 {
        surrounding.push((x, y - 1));
        surrounding.push((x + 1, y - 1));
    }
    
    if x > 0 && y > 0 {
        surrounding.push((x - 1, y - 1));
    }
    
    surrounding.push((x, y + 1));
    surrounding.push((x + 1, y));
    surrounding.push((x + 1, y + 1));
    
    surrounding.iter().filter(|&&(xpos, ypos)| is_mine(positions, xpos, ypos) ).count() as u32
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldView{
    view: Vec<Vec<Tile>>,
    field: Field,
    game_over: bool
}

impl FieldView {
    pub fn new(width:u32, height:u32, mines:usize) -> FieldView {
    
        let mut tiles = vec!();
        for _ in 0 .. height {
            let mut row = vec!();
            for _ in 0 .. width {
                row.push(Tile::Closed);
            }
            
            tiles.push(row);
        }
        
        FieldView{view: tiles, field: Field::new(width, height, mines), game_over:false}
    }
    
    pub fn in_play(&self) -> bool {
        
        self.game_over == false
    }
    
    pub fn select(&mut self, col:String, row:u32){
        let c = FieldView::get_col_num(col) as usize;
        let r = row as usize;
        if r < self.view.len() && c < self.view[0].len() {
            self.view[r][c] = Tile::Open;
            
            if self.field.0[r][c] == TileState::Mine {
                self.game_over = true;
            }
        }
    }
    
    fn get_col_num(col:String) -> u32 {
        match col.trim() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            "i" => 8,
            "j" => 9,
            "k" => 10,
            _   => 11
        }
    }
}

impl fmt::Display for FieldView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let _ = write!(f, " ");
        let letters = vec!('a', 'b', 'c', 
            'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 
            'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
            
        let mut cols = letters.iter();

        let _ = write!(f, " ");
        for _ in 0..self.view.iter().count() {
            let _ = write!(f, "{} ", cols.next().unwrap());
        }
        let _ = write!(f, "\n");
        
        let mut row_num = 0;
        let mut col_num = 0;
        for row in self.view.iter(){
            let _ = write!(f, "{} ", row_num);
            col_num = 0;
            for tile in row.iter(){
                let _ = match *tile {
                    Tile::Closed => write!(f, "{}", tile),
                    Tile::Open   => write!(f, "{}", self.field.0[row_num][col_num]),
                    _            => write!(f, "?")
                };
                col_num = col_num + 1;
            }
            let _ = write!(f, "\n");
            row_num = row_num + 1;
        }
        writeln!(f, "\n")
        
    }
}