use std::{
    cell::RefCell,
    io::{self, Write},
};

use rand::{thread_rng, Rng};

use crate::{
    gamecell::{CellType, GameCell},
    player::Player,
    wumpus::Wumpus,
};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading stdin");
    input.trim().to_owned()
}

pub struct Grid {
    cells: RefCell<Vec<Vec<GameCell>>>,
    player: RefCell<Player>,
    wumpus: RefCell<Wumpus>,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let mut cells = Vec::with_capacity(5);
        for _ in 0..5 {
            let mut v = Vec::with_capacity(5);
            for _ in 0..5 {
                v.push(GameCell::new(CellType::Empty));
            }
            cells.push(v);
        }

        loop {
            let row = rng.gen_range(0, 5);
            let col = rng.gen_range(0, 5);

            if let CellType::Empty = cells[row][col].celltype() {
                cells[row][col] = GameCell::new(CellType::Arrow);
                break;
            }
        }
        loop {
            let row = rng.gen_range(0, 5);
            let col = rng.gen_range(0, 5);

            if let CellType::Empty = cells[row][col].celltype() {
                cells[row][col] = GameCell::new(CellType::Bats);
                break;
            }
        }
        loop {
            let row = rng.gen_range(0, 5);
            let col = rng.gen_range(0, 5);

            if let CellType::Empty = cells[row][col].celltype() {
                cells[row][col] = GameCell::new(CellType::Pit);
                break;
            }
        }
        let mut player = Player::new((0, 0));
        let mut wumpus = Wumpus::new((0, 0));
        loop {
            player.set_pos((rng.gen_range(0, 5), rng.gen_range(0, 5)));
            wumpus.set_pos((rng.gen_range(0, 5), rng.gen_range(0, 5)));

            if player.pos() != wumpus.pos() {
                break;
            }
        }

        Self {
            cells: RefCell::new(cells),
            player: RefCell::new(player),
            wumpus: RefCell::new(wumpus),
        }
    }

    fn show_grid(&self) {
        for (y, row) in self.cells.borrow().iter().enumerate() {
            for (x, gc) in row.iter().enumerate() {
                if (x, y) == self.player.borrow().pos() {
                    print!("@ ");
                } else {
                    print!("{} ", gc.symbol());
                }
            }
            println!();
        }
    }

    fn ask(&self, prompt: &str) -> String {
        loop {
            print!("{}", prompt);
            io::stdout().flush().expect("Error flushing stdout");
            let input = read_line();
            if !input.is_empty() {
                return input;
            }
        }
    }

    fn sense(&self) {
        for (y, row) in self.cells.borrow().iter().enumerate() {
            for (x, gc) in row.iter().enumerate() {
                if self.player.borrow().near((x, y)) {
                    match gc.celltype() {
                        CellType::Bats => println!("You hear flapping."),
                        CellType::Pit => println!("You feel a breeze."),
                        _ => (),
                    }
                }
            }
        }
        if self.player.borrow().near(self.wumpus.borrow().pos()) {
            println!("You smell a wumpus.");
        }
    }

    fn fire_arrow(&self, direction: (i32, i32)) {
        if self.player.borrow().can_move(direction) {
            self.player.borrow_mut().fire();
            let target = (
                (self.player.borrow().pos().0 as i32 + direction.0) as usize,
                (self.player.borrow().pos().1 as i32 + direction.1) as usize,
            );
            if self.wumpus.borrow().pos() == target {
                println!("You shot and killed the wumpus! You win!!!");
                self.player.borrow_mut().die();
            } else {
                println!("You missed!");
            }
            self.wumpus.borrow_mut().move_random();
        } else {
            println!("You cannot fire an arrow that way.");
        }
    }

    fn check_spot(&self) {
        if self.player.borrow().pos() == self.wumpus.borrow().pos() {
            println!("The fearsome wumpus ate you! You died.");
            self.player.borrow_mut().die();
        } else {
            let curr_cell = self.cells.borrow()[self.player.borrow().pos().1]
                [self.player.borrow().pos().0]
                .celltype();
            match curr_cell {
                CellType::Arrow => {
                    println!("You picked up an arrow!");
                    self.player.borrow_mut().pick_up_arrow();
                    self.cells.borrow_mut()[self.player.borrow().pos().1]
                        [self.player.borrow().pos().0] = GameCell::new(CellType::Empty);
                }
                CellType::Bats => {
                    println!("The bats carried you away!");
                    self.player.borrow_mut().move_rand();
                    self.check_spot();
                }
                CellType::Pit => {
                    println!("You fell down a pit! You died.");
                    self.player.borrow_mut().die();
                }
                _ => (),
            }
        }
    }

    pub fn play(&self) {
        println!("Rules:\n\n\
            There are three traps:\n\
            A bat swarm that will pick you up and drop you in a random place (you will hear flapping)\n\
            A deadly pit (you will feel a breeze)\n\
            A dangerous, moving wumpus (you will smell it).\n\n\
            You have five arrows.\n\
            You may find an extra arrow.\n");

        while self.player.borrow().is_alive() {
            println!();

            self.cells.borrow_mut()[self.player.borrow().pos().1][self.player.borrow().pos().0]
                .discover();

            self.show_grid();

            self.player.borrow().show_arrows();

            self.sense();

            let action = self.ask("Move or Shoot? (m/s): ");
            let direction = self.ask("What direction? (n/s/e/w): ");

            let direction = match direction.as_str() {
                "n" => (0, -1),
                "s" => (0, 1),
                "e" => (1, 0),
                "w" => (-1, 0),
                _ => {
                    println!("Invalid direction.");
                    continue;
                }
            };

            if action == "m" {
                if self.player.borrow().can_move(direction) {
                    self.player.borrow_mut().move_self(direction);
                } else {
                    println!("You cannot move that way.");
                }
            } else if action == "s" {
                if self.player.borrow().has_arrows() {
                    self.fire_arrow(direction);
                } else {
                    println!("You ran out of arrows. The wumpus eats you for lunch. You died.");
                    self.player.borrow_mut().die();
                }
            } else {
                println!("Invalid action.");
            }

            self.check_spot();
        }
    }
}
