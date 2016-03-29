use std::fmt;

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Flag,
    Closed,
    Open
}

impl fmt::Display for Tile{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", "█")
    }
}



#[derive(Clone, Debug, PartialEq)]
pub struct Field(Vec<Vec<Tile>>);

impl Field {
    pub fn new(width:u8, height:u8) -> Field {
    
        let mut tiles = vec!();
        for _ in 0 .. height {
            let mut row = vec!();
            for _ in 0 .. width {
                row.push(Tile::Closed);
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

        for _ in 0..self.0.iter().count() {
            write!(f, "{} ", cols.next().unwrap());
        }
        write!(f, "\n");
        
        let mut row_num = 0;
        for row in self.0.iter(){
            write!(f, "{}", row_num);
            for tile in row.iter(){
                write!(f, "{}", tile);
            }
            write!(f, "\n");
            row_num = row_num + 1;
        }
        writeln!(f, "\n")
        
    }
}