use crate::beverage::Beverage;
use crate::board::Place;
use crate::event::{ActionType, Event};
use crate::team::Team;
use rand::Rng;
use std::cell::Cell;
use std::cmp::min;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};

pub struct Game {
    pub board: Vec<Arc<Mutex<Place>>>,
    pub teams: Vec<Arc<Team>>,
    pub time: Cell<f32>,
    pub event_queue: BinaryHeap<Event>,
}

impl Game {
    pub fn new(teams: Vec<Arc<Team>>, board: Vec<Arc<Mutex<Place>>>) -> Self {
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
                self.board[team.location.get()].lock().unwrap().name,
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
            println!("-----");
            println!("Processing event: {}", event);
            self.handle_event(event);
            println!("-----");
        }
    }

    fn schedule_event(&mut self, event: Event) {
        println!("-----");
        println!("Scheduling event: {}", event);
        self.event_queue.push(event);
        println!("Scheduled. Event queue size: {}", self.event_queue.len());
        println!("-----");
    }

    fn handle_event(&mut self, event: Event) {
        self.time.set(event.timestamp);

        match event.action {
            ActionType::EmptyPlace {} => {
                let location_index = event.team.location.get();
                let place = Arc::clone(&self.board[location_index]);

                let mut place_guard = place.lock().unwrap();

                while let Some(beverage) = place_guard.beverages.pop() {
                    self.assign_beverage(Arc::clone(&event.team), beverage);
                }

                place_guard.beverages = place_guard.refills.clone();
            }

            ActionType::RollDice {} => {
                self.roll_dice(Arc::clone(&event.team));
                self.schedule_event(Event {
                    team: Arc::clone(&event.team),
                    timestamp: self.time.get() + event.duration,
                    duration: rand::thread_rng().gen_range(1.0..10.0),
                    action: ActionType::EmptyPlace {},
                });
            }
            ActionType::ThrowUp { player } => {
                println!("{} throws up!", player.lock().unwrap().name);
            }
            ActionType::DrinkBeverage { player, beverage } => {
                let player_name;
                {
                    let player_guard = player.lock().unwrap();
                    player_name = player_guard.name.clone();
                }

                println!(
                    "{} from {} drinks a {}",
                    player_name, event.team.name, beverage.name
                );

                let success;
                {
                    let mut player = player.lock().unwrap();
                    success = player.drink(beverage.clone(), self.time.get());
                }

                if !success {
                    self.schedule_event(Event {
                        team: Arc::clone(&event.team),
                        timestamp: self.time.get() + event.duration / 2.0,
                        duration: rand::thread_rng().gen_range(5.0..60.0),
                        action: ActionType::ThrowUp {
                            player: player.clone(),
                        },
                    });
                }
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

    // TODO: beverage sharing strategies
    fn assign_beverage(&mut self, team: Arc<Team>, beverage: Beverage) {
        let player = &team.players[0];

        self.schedule_event(Event {
            team: Arc::clone(&team),
            timestamp: self.time.get(),
            duration: rand::thread_rng().gen_range(8.0..60.0),
            action: ActionType::DrinkBeverage {
                player: Arc::clone(&player),
                beverage: beverage.clone(),
            },
        });
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
        println!("-----");
    }
}
