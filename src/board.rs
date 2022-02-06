use crate::card::Card;

#[derive(PartialEq, Debug)]
pub struct Board {
    pub cards: Vec<Card>,
}

// impl PartialEq for Board {
//     fn eq(&self, other: &Self) -> bool {
//         if self.cards.len() != other.cards.len() {
//             return false;
//         }
//
//         for i in 0..self.cards.len() {
//             if self.cards[i] != other.cards[i] {
//                 return false;
//             }
//         }
//
//         return true;
//     }
// }

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