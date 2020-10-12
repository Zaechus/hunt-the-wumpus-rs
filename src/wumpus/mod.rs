use rand::{thread_rng, Rng};

pub struct Wumpus {
    pos: (usize, usize),
}

impl Wumpus {
    pub fn new(pos: (usize, usize)) -> Self {
        Self { pos }
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.pos = pos;
    }

    pub fn move_random(&mut self) {
        let mut rng = thread_rng();

        self.pos = (rng.gen_range(0, 5), rng.gen_range(0, 5));
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }
}
