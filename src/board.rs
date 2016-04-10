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
    Number(u8)
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
    pub fn new(width:u8, height:u8, mines:usize) -> Field {

        let mut rng = rand::thread_rng();
        let width_range = (0..width).collect::<Vec<u8>>();
        let height_range = (0..height).collect::<Vec<u8>>();
        
        // get mine positions
        let mine_positions = (0..mines).map(|_| 
                (
                    rng.choose(&width_range).unwrap().clone(), 
                    rng.choose(&height_range).unwrap().clone())
                ).collect::<Vec<(u8, u8)>>();


        // create complete board
        let mut tiles = vec!();
        for y in 0 .. height {
            let mut row = vec!();
            for x in 0 .. width {
                let tile = if isMine(&mine_positions, x, y) {
                    TileState::Mine
                }else{
                    TileState::Number(countMines(&mine_positions, x, y))
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
    
        write!(f, " ");
        let letters = vec!('a', 'b', 'c', 
            'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 
            'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
            
        let mut cols = letters.iter();

        write!(f, " ");
        for _ in 0..self.0.iter().count() {
            write!(f, "{} ", cols.next().unwrap());
        }
        write!(f, "\n");
        
        let mut row_num = 0;
        for row in self.0.iter(){
            write!(f, "{} ", row_num);
            for tile in row.iter(){
                write!(f, "{}", tile);
            }
            write!(f, "\n");
            row_num = row_num + 1;
        }
        writeln!(f, "\n")
        
    }
}

fn isMine(positions: &Vec<(u8, u8)>, x: u8, y: u8) -> bool {
    positions.iter().any(|&(xpos, ypos)| xpos == x && ypos == y)
}

fn countMines(positions: &Vec<(u8, u8)>, x: u8, y: u8) -> u8 {
    let mut surrounding:Vec<(u8, u8)> = vec!();
    
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
    
    surrounding.iter().filter(|&&(xpos, ypos)| isMine(positions, xpos, ypos) ).count() as u8
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldView{
    view: Vec<Vec<Tile>>,
    field: Field
}

impl FieldView {
    pub fn new(width:u8, height:u8, mines:usize) -> FieldView {
    
        let mut tiles = vec!();
        for _ in 0 .. height {
            let mut row = vec!();
            for _ in 0 .. width {
                row.push(Tile::Closed);
            }
            
            tiles.push(row);
        }
        
        FieldView{view: tiles, field: Field::new(width, height, mines)}
    }
    
    pub fn showResult(&self) {
        println!("{}", self.field);
    }
}

impl fmt::Display for FieldView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        write!(f, " ");
        let letters = vec!('a', 'b', 'c', 
            'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 
            'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z');
            
        let mut cols = letters.iter();

        write!(f, " ");
        for _ in 0..self.view.iter().count() {
            write!(f, "{} ", cols.next().unwrap());
        }
        write!(f, "\n");
        
        let mut row_num = 0;
        for row in self.view.iter(){
            write!(f, "{} ", row_num);
            for tile in row.iter(){
                write!(f, "{}", tile);
            }
            write!(f, "\n");
            row_num = row_num + 1;
        }
        writeln!(f, "\n")
        
    }
}