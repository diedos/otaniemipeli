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
use std::sync::Arc;
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
        Place {
            name: "Start".to_string(),
            beverages: Some(vec![slime.clone()]),
            refilled: false,
        },
        Place {
            name: "Middle1".to_string(),
            beverages: Some(vec![beer.clone()]),
            refilled: false,
        },
        Place {
            name: "Middle2".to_string(),
            beverages: Some(vec![beer.clone()]),
            refilled: true,
        },
        Place {
            name: "Middle3".to_string(),
            beverages: Some(vec![beer.clone()]),
            refilled: false,
        },
        Place {
            name: "Middle4".to_string(),
            beverages: Some(vec![beer.clone()]),
            refilled: false,
        },
        Place {
            name: "Middle5".to_string(),
            beverages: Some(vec![beer.clone()]),
            refilled: false,
        },
        Place {
            name: "Middle6".to_string(),
            beverages: Some(vec![beer.clone()]),
            refilled: false,
        },
        Place {
            name: "Middle7".to_string(),
            beverages: Some(vec![beer.clone()]),
            refilled: false,
        },
        Place {
            name: "Middle8".to_string(),
            beverages: Some(vec![beer.clone()]),
            refilled: false,
        },
        Place {
            name: "End".to_string(),
            beverages: None,
            refilled: false,
        },
    ];

    let teams = vec![
        Arc::new(Team {
            name: "Team 1".to_string(),
            players: vec![Player {
                name: "Player 1".to_string(),
            }],
            location: Cell::new(0),
            rolled_double: Cell::new(false),
        }),
        Arc::new(Team {
            name: "Team 2".to_string(),
            players: vec![Player {
                name: "Player 2".to_string(),
            }],
            location: Cell::new(0),
            rolled_double: Cell::new(false),
        }),
        Arc::new(Team {
            name: "Team 3".to_string(),
            players: vec![Player {
                name: "Player 3".to_string(),
            }],
            location: Cell::new(0),
            rolled_double: Cell::new(false),
        }),
        Arc::new(Team {
            name: "Team 4".to_string(),
            players: vec![Player {
                name: "Player 4".to_string(),
            }],
            location: Cell::new(0),
            rolled_double: Cell::new(false),
        }),
    ];

    let mut game = Game::new(teams, board);
    game.start();
}
