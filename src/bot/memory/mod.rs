mod node;
mod list;
mod memory;
#[derive(Clone, PartialEq)]
pub enum Status {
    Progress,
    Win,
    Lose,
    Draw
}