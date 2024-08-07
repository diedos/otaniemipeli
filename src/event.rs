use crate::beverage::Beverage;
use crate::team::{Player, Team};
use std::cmp::Ordering;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum ActionType {
    EmptyPlace {},
    RollDice {},
    ThrowUp {
        player: Arc<Mutex<Player>>,
    },
    DrinkBeverage {
        player: Arc<Mutex<Player>>,
        beverage: Beverage,
    },
}

#[derive(Debug)]
pub struct Event {
    pub team: Arc<Team>,
    pub timestamp: f32,
    pub duration: f32,
    pub action: ActionType,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.partial_cmp(&self.timestamp).unwrap()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Event: team={}, timestamp={}, action={:?}, duration={}",
            self.team.name, self.timestamp, self.action, self.duration
        )
    }
}
