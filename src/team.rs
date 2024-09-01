use std::cell::Cell;
use std::sync::{Arc, Mutex};

use crate::beverage::{Beverage, BeverageHistoryItem};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub beverage_history: Vec<BeverageHistoryItem>,
    pub liquid_capacity: u32,
    pub liquid_content: Cell<u32>,
}

impl Player {
    /// Drink a beverage and handle all player internal state changes.
    /// Returns a tuple (drank it successfully, time to drink).
    /// Unsuccessful drinking should trigger a ThrowUp event.
    pub fn drink(&mut self, beverage: Beverage, timestamp: f32) -> bool {
        self.beverage_history.push(BeverageHistoryItem {
            beverage: beverage.clone(),
            timestamp,
        });

        self.liquid_content
            .set(self.liquid_content.get() + beverage.size_ml);

        let player_overflowing = self.liquid_content.get() > self.liquid_capacity;

        !player_overflowing
    }
}

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub players: Vec<Arc<Mutex<Player>>>,
    pub location: Cell<usize>,
    pub rolled_double: Cell<bool>,
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
