use crate::card::Card;

#[derive(PartialEq, Debug)]
pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn new(cards: Vec<Card>) -> Board {
        if cards.len() > 5 {
            panic!("6枚以上のカードをボード上に並べることはできません");
        }

        Board {
            cards
        }
    }
}