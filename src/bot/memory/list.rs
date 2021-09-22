use crate::bot::memory::node::Node;

enum Link {
    None,
    Next(Box<List>)
}

pub struct List {
    list: (Vec<Node>, Link)
}