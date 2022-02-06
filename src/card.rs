use crate::suit::Suit;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Card {
    pub number: u8,
    pub suit: Suit,
    pub state: State,
}

impl Card {
    pub fn new(number: u8, suit: Suit, state: State) -> Card {
        if number < 1 || 13 < number {
            panic!("トランプの数字は1以上13以下でなければなりません。")
        }

        Card { number, suit, state }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum State {
    // デッキ中にある
    InDeck,

    // ハンド中にある
    InHand,

    // ボード上にある
    OnBoard,
}