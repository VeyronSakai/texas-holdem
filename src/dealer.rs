use crate::card::Card;
use crate::card::State::InDeck;
use crate::suit::Suit;

#[derive(PartialEq, Debug)]
pub struct Dealer {
    pub deck: Vec<Card>,
}

impl Dealer {
    pub fn new() -> Dealer {
        let mut deck = Vec::new();
        for num in 1..=13 {
            deck.push(Card::new(num, Suit::Club, InDeck));
            deck.push(Card::new(num, Suit::Heart, InDeck));
            deck.push(Card::new(num, Suit::Diamond, InDeck));
            deck.push(Card::new(num, Suit::Spade, InDeck));
        }

        Dealer {
            deck
        }
    }
}
