use rand::{thread_rng, Rng};

enum Status {
    Alive,
    Dead,
}

pub struct Player {
    pos: (usize, usize),
    status: Status,
    arrows: u32,
}

impl Player {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            status: Status::Alive,
            arrows: 5,
        }
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

    pub fn show_arrows(&self) {
        println!("\nArrows: {}", self.arrows);
    }

    pub fn can_move(&self, direction: (i32, i32)) -> bool {
        let new_x = self.pos.0 as i32 + direction.0;
        let new_y = self.pos.1 as i32 + direction.1;

        new_x >= 0 && new_x <= 5 && new_y >= 0 && new_y <= 5
    }

    pub fn move_self(&mut self, direction: (i32, i32)) {
        let new_x = self.pos.0 as i32 + direction.0;
        let new_y = self.pos.1 as i32 + direction.1;
        self.pos.0 = new_x as usize;
        self.pos.1 = new_y as usize;
    }

    pub fn move_rand(&mut self) {
        let mut rng = thread_rng();
        self.pos = (rng.gen_range(0, 5), rng.gen_range(0, 5));
    }

    pub fn fire(&mut self) {
        if self.has_arrows() {
            self.arrows -= 1;
        }
    }

    pub fn pick_up_arrow(&mut self) {
        self.arrows += 1;
    }

    pub fn die(&mut self) {
        self.status = Status::Dead;
    }

    pub fn near(&self, other: (usize, usize)) -> bool {
        (other.0 as i32 - self.pos.0 as i32).abs() <= 1
            && (other.1 as i32 - self.pos.1 as i32).abs() <= 1
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn has_arrows(&self) -> bool {
        self.arrows > 0
    }

    pub fn is_alive(&self) -> bool {
        matches!(self.status, Status::Alive)
    }
}
