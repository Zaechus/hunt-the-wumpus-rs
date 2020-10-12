pub enum Visited {
    Seen,
    Unseen,
}

#[derive(Copy, Clone)]
pub enum CellType {
    Empty,
    Arrow,
    Bats,
    Pit,
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

    pub fn celltype(&self) -> CellType {
        self.celltype
    }

    pub fn symbol(&self) -> char {
        if let Visited::Seen = self.visited {
            'O'
        } else {
            '_'
        }
    }

    pub fn discover(&mut self) {
        self.visited = Visited::Seen;
    }
}
