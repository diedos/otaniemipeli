use std::cell::Cell;

#[derive(Debug)]
pub struct Player {
    pub name: String,
}

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
    pub location: Cell<usize>,
    pub rolled_double: Cell<bool>,
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
