use crate::lib::{gen_mines, position::Position};

pub struct Game {
    pub mines: Vec<Position>,
}

impl Game {
    pub fn new_mines(&mut self) {
        self.mines = gen_mines();
    }
}
