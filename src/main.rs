mod args;
mod beverage;
mod board;
mod event;
mod game;
mod team;

use args::Args;
use beverage::{Beverage, BeverageType};
use board::Place;
use clap::Parser;
use game::Game;
use std::cell::Cell;
use std::sync::{Arc, Mutex};
use team::{Player, Team};

fn main() {
    let args = Args::parse();

    let beer = Beverage {
        name: "Beer".to_string(),
        flavor: BeverageType::Beer,
        size_ml: 330,
        abv: 0.047,
    };

    let slime = Beverage {
        name: "Slime".to_string(),
        flavor: BeverageType::Slime,
        size_ml: 40,
        abv: 0.35,
    };

    let board = vec![
        Arc::new(Mutex::new(Place {
            name: "Start".to_string(),
            beverages: vec![slime.clone()],
            refills: vec![],
        })),
        Arc::new(Mutex::new(Place {
            name: "Middle1".to_string(),
            beverages: vec![beer.clone()],
            refills: vec![beer.clone()],
        })),
        Arc::new(Mutex::new(Place {
            name: "Middle2".to_string(),
            beverages: vec![beer.clone()],
            refills: vec![beer.clone()],
        })),
        Arc::new(Mutex::new(Place {
            name: "Middle3".to_string(),
            beverages: vec![beer.clone()],
            refills: vec![beer.clone()],
        })),
        Arc::new(Mutex::new(Place {
            name: "Middle4".to_string(),
            beverages: vec![beer.clone()],
            refills: vec![beer.clone()],
        })),
        Arc::new(Mutex::new(Place {
            name: "Middle5".to_string(),
            beverages: vec![beer.clone()],
            refills: vec![],
        })),
        Arc::new(Mutex::new(Place {
            name: "End".to_string(),
            beverages: vec![],
            refills: vec![],
        })),
    ];

    let teams = vec![
        Arc::new(Team {
            name: "Team 1".to_string(),
            players: vec![Arc::new(Mutex::new(Player {
                name: "Player 1".to_string(),
                beverage_history: Vec::new(),
                liquid_capacity: 800,
                liquid_content: Cell::new(0),
            }))],
            location: Cell::new(0),
            rolled_double: Cell::new(false),
        }),
        Arc::new(Team {
            name: "Team 2".to_string(),
            players: vec![Arc::new(Mutex::new(Player {
                name: "Player 2".to_string(),
                beverage_history: Vec::new(),
                liquid_capacity: 1000,
                liquid_content: Cell::new(0),
            }))],
            location: Cell::new(0),
            rolled_double: Cell::new(false),
        }),
        Arc::new(Team {
            name: "Team 3".to_string(),
            players: vec![Arc::new(Mutex::new(Player {
                name: "Player 3".to_string(),
                beverage_history: Vec::new(),
                liquid_capacity: 1200,
                liquid_content: Cell::new(0),
            }))],
            location: Cell::new(0),
            rolled_double: Cell::new(false),
        }),
        Arc::new(Team {
            name: "Team 4".to_string(),
            players: vec![Arc::new(Mutex::new(Player {
                name: "Player 4".to_string(),
                beverage_history: Vec::new(),
                liquid_capacity: 1400,
                liquid_content: Cell::new(0),
            }))],
            location: Cell::new(0),
            rolled_double: Cell::new(false),
        }),
    ];

    let mut game = Game::new(teams, board, args.seed);
    game.start();
}
