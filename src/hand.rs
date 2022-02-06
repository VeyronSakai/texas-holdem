use std::collections::BTreeSet;
use crate::card::Card;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Hand {
    pub card1: Card,
    pub card2: Card,
}

impl Hand {
    pub fn new(card1: Card, card2: Card) -> Hand {
        if card1 == card2 {
            panic!("全く同じカードが配られることはありません。")
        }

        Hand { card1, card2 }
    }
}


