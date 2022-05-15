use crate::domain::card::{Card, State};
use crate::domain::suit::Suit;

pub const HAND_NUM: usize = 2;

#[derive(PartialEq, Debug)]
pub struct Player {
    pub cards: [Card; HAND_NUM],
    pub chip: i32,
}

impl Player {
    pub fn new(cards: [Card; HAND_NUM], chip: i32) -> Player {
        Player {
            cards,
            chip,
        }
    }
}
