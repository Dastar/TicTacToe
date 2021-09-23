mod node;
mod list;
pub mod memory;
#[derive(Clone, PartialEq)]
pub enum Status {
    Progress,
    Win,
    Lose,
    Draw
}