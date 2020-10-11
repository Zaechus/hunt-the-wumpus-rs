use crate::gamecell::{CellType, GameCell};
use crate::player::Player;

pub struct Grid {
    cells: Vec<Vec<GameCell>>,
}

impl Grid {
    pub fn new() -> Self {
        let mut cells = Vec::with_capacity(5);
        for _ in 0..5 {
            let mut v = Vec::with_capacity(5);
            for _ in 0..5 {
                v.push(GameCell::new(CellType::Empty));
            }
            cells.push(v);
        }
        cells[1][1] = GameCell::new(CellType::Player(Player::new()));
        cells[1][1].discover();
        Self { cells }
    }

    pub fn show_grid(&self) {
        for y in self.cells.iter() {
            for gc in y {
                print!("{} ", gc.symbol());
            }
            println!();
        }
    }
}
