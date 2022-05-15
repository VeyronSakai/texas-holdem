use crate::domain::card::Card;
use crate::domain::player::{HAND_NUM, Player};

pub trait PlayerFactory {
    fn create(&self, hands: [Card; HAND_NUM], chip: i32) -> Player;
}