use crate::player::Player;

pub enum Visited {
    Seen,
    Unseen,
}

pub enum CellType {
    Empty,
    Bats,
    Pit,
    Player(Player),
    Wumpus,
}

pub struct GameCell {
    celltype: CellType,
    visited: Visited,
}

impl GameCell {
    pub fn new(celltype: CellType) -> Self {
        Self {
            celltype,
            visited: Visited::Unseen,
        }
    }

    pub fn symbol(&self) -> char {
        if let Visited::Seen = self.visited {
            match self.celltype {
                CellType::Bats => 'B',
                CellType::Pit => '^',
                CellType::Player(_) => '@',
                CellType::Wumpus => 'W',
                _ => 'O',
            }
        } else {
            '_'
        }
    }

    pub fn discover(&mut self) {
        self.visited = Visited::Seen;
    }
}
