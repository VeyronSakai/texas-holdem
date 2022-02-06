use crate::card::Card;

pub const HAND_NUM: usize = 2;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Hand {
    pub cards: [Card; HAND_NUM],
}

impl Hand {
    pub fn new(cards: [Card; HAND_NUM]) -> Hand {
        if cards[0] == cards[1] {
            panic!("全く同じカードが配られることはありません。")
        }

        Hand { cards }
    }
}
