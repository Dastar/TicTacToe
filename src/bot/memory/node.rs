use crate::bot::memory::Status;

const COST: i32 = 1;
const START_WEIGHT: i32 = 100;

pub struct Node {
    status: Status,
    movement: usize,
    weight: i32,
}

impl Node {
    fn new(status: Status, movement: usize) -> Self {
        let weight = START_WEIGHT;
        Node { status, movement, weight }
    }

    fn value(&self) -> usize {
        self.movement
    }

    fn increse_weight(&mut self) {
        self.weight += COST;
    }

    fn decrese_weight(&mut self) {
        self.weight -= COST;
    }
}
