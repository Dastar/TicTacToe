use crate::bot::memory::Status;

const COST: i32 = 1;
const START_WEIGHT: i32 = 100;

pub struct Node {
    pub status: Status,
    pub movement: usize,
    pub active: bool,
    pub weight: i32,
}

impl Node {
    pub fn new(status: Status, movement: usize) -> Self {
        let weight = START_WEIGHT;
        let active = false;
        Node { status, movement, active, weight }
    }

    pub fn won(&mut self) {
        self.active = false;
        self.weight += COST;
    }

    pub fn lost(&mut self) {
        self.active = false;
        self.weight -= 2 * COST;
    }

    pub fn draw(&mut self) {
        self.active = false;
        self.weight -= COST;
    }
}
