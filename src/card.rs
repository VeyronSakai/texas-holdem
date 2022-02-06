use crate::suit::Suit;

#[derive(PartialEq, Debug)]
pub struct Card {
    pub number: u8,
    pub suit: Suit,
}

impl Card {
    pub fn new(number: u8, suit: Suit) -> Card {
        if number < 1 || 13 < number {
            panic!("トランプの数字は1以上13以下でなければなりません。")
        }

        Card { number, suit }
    }
}
