pub mod player_id;
pub mod player_factory;
pub mod player_repository;

use uuid::Uuid;
use crate::domain::card::{Card, State};
use crate::domain::player::player_id::PlayerId;
use crate::domain::suit::Suit;

pub const HAND_NUM: usize = 2;

#[derive(PartialEq, Debug, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub cards: [Card; HAND_NUM],
    pub chip: i32,
}

impl Player {
    pub fn new(id: PlayerId, cards: [Card; HAND_NUM], chip: i32) -> Player {
        Player {
            id,
            cards,
            chip,
        }
    }
}
