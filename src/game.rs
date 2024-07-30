use crate::board::Place;
use crate::event::{ActionType, Event};
use crate::team::Team;
use rand::Rng;
use std::cell::Cell;
use std::cmp::min;
use std::collections::BinaryHeap;
use std::sync::Arc;

pub struct Game {
    pub board: Vec<Place>,
    pub teams: Vec<Arc<Team>>,
    pub time: Cell<f32>,
    pub event_queue: BinaryHeap<Event>,
}

impl Game {
    pub fn new(teams: Vec<Arc<Team>>, board: Vec<Place>) -> Self {
        Self {
            board,
            teams,
            time: Cell::new(0.0),
            event_queue: BinaryHeap::new(),
        }
    }

    pub fn start(&mut self) {
        let teams: Vec<_> = self.teams.iter().cloned().collect();
        for team in teams {
            self.schedule_event(Event {
                team: Arc::clone(&team),
                timestamp: rand::thread_rng().gen_range(3.0..10.0),
                duration: rand::thread_rng().gen_range(10.0..45.0),
                action: ActionType::RollDice {},
            });
        }

        while self.is_active() {
            self.print_event_queue();
            self.process_next_event();
        }

        println!("Game over!");
        println!("Final locations:");
        for team in &self.teams {
            println!(
                "{}: {} ({})",
                team.name,
                self.board[team.location.get()].name,
                team.location.get()
            );
        }
    }

    fn is_active(&self) -> bool {
        self.teams
            .iter()
            .all(|team| team.location.get() < self.board.len() - 1)
    }

    fn process_next_event(&mut self) {
        if let Some(event) = self.event_queue.pop() {
            println!("Processing event: {}", event);
            self.handle_event(event);
        }
    }

    fn schedule_event(&mut self, event: Event) {
        println!("Scheduling event: {}", event);
        self.event_queue.push(event);
    }

    fn handle_event(&mut self, event: Event) {
        self.time.set(event.timestamp);

        match event.action {
            ActionType::Drink { player, beverage } => {
                println!("{} should drink a {}!", player.name, beverage.name);
            }
            ActionType::RollDice {} => {
                self.roll_dice(Arc::clone(&event.team));
            }
            ActionType::ThrowUp { player } => {
                println!("{} throws up!", player.name);
            }
        }

        // If the team doesn't have anything on their table, they want to roll the dice
        // If the team has already won, don't schedule another dice roll
        if !self
            .event_queue
            .iter()
            .any(|queued_event| queued_event.team == event.team)
            && event.team.location.get() != self.board.len() - 1
        {
            self.schedule_event(Event {
                team: Arc::clone(&event.team),
                timestamp: self.time.get() + event.duration,
                duration: rand::thread_rng().gen_range(10.0..45.0),
                action: ActionType::RollDice {},
            });
        }
    }

    fn roll_dice(&mut self, team: Arc<Team>) {
        let dice_roll = (
            rand::thread_rng().gen_range(1..7),
            rand::thread_rng().gen_range(1..7),
        );

        println!(
            "{} rolls a {} and a {}",
            team.name, dice_roll.0, dice_roll.1
        );

        if dice_roll.0 == dice_roll.1 {
            println!("{} rolled a double!", team.name);
            team.rolled_double.set(true);
        }

        let old_location = team.location.get();
        let board_len = self.board.len();
        let new_location = team.location.get() + min(dice_roll.0, dice_roll.1);
        team.location.set(if new_location >= board_len {
            println!("{} almost won the game. Board overflow :(", team.name);
            board_len - 2 - (new_location - board_len)
        } else {
            new_location
        });

        println!(
            "{} moved from {} to {}",
            team.name,
            old_location,
            team.location.get()
        );

        if new_location == board_len - 1 {
            println!("{} won the game!", team.name);
        }
    }

    fn print_event_queue(&self) {
        println!("Event Queue:");
        for event in &self.event_queue {
            println!("- {}", event);
        }
    }
}
