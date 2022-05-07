use crate::card::{Card, State};
use crate::suit::Suit;

const HAND_NUM: usize = 2;

#[derive(PartialEq, Debug)]
pub struct Player {
    pub cards: [Card; HAND_NUM],
    pub chip: i32,
}

impl Player {
    pub fn new(cards: [Card; HAND_NUM]) -> Player {
        Player {
            cards,
            chip: 0,
        }
    }
}
