use crate::domain::card::Card;
use crate::domain::player::{HAND_NUM, Player};

pub trait PlayerFactory {
    fn create(&self, chip: i32) -> Player;
}