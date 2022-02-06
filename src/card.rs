use crate::suit::Suit;

#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum State {
    // デッキ中にある
    InDeck,

    // ハンド中にある
    InHand,

    // 表向きの状態でボード上にある
    UpOnBoard,

    // 裏向きの状態でボード上にある
    DownOnBoard,
}