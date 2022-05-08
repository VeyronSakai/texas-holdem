use rand::prelude::SliceRandom;
use crate::domain::card::Card;
use crate::domain::card::State::InDeck;
use crate::domain::suit::Suit;

#[derive(PartialEq, Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();

        for num in 1..=13 {
            cards.push(Card::new(num, Suit::Club, InDeck));
            cards.push(Card::new(num, Suit::Heart, InDeck));
            cards.push(Card::new(num, Suit::Diamond, InDeck));
            cards.push(Card::new(num, Suit::Spade, InDeck));
        }

        Deck {
            cards
        }
    }

    // Shuffle deck
    pub fn shuffle_cards(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

#[test]
fn new_test() {
    #[derive(Debug)]
    struct TestCase {
        name: String,
        expected: Deck,
    }

    let table = [
        TestCase {
            expected: crate::domain::deck::tests::build_mock_deck(),
            name: String::from("正常系1"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Deck::new(),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}

#[test]
fn shuffle_test() {
    let mut deck = Deck::new();
    deck.shuffle_cards();
}

#[cfg(test)]
pub mod tests {
    use crate::domain::card::Card;
    use crate::domain::card::State::InDeck;
    use crate::domain::deck::Deck;
    use crate::domain::suit::Suit;

    pub fn build_mock_deck() -> Deck {
        let cards = vec![
            Card::new(1, Suit::Club, InDeck),
            Card::new(1, Suit::Heart, InDeck),
            Card::new(1, Suit::Diamond, InDeck),
            Card::new(1, Suit::Spade, InDeck),
            Card::new(2, Suit::Club, InDeck),
            Card::new(2, Suit::Heart, InDeck),
            Card::new(2, Suit::Diamond, InDeck),
            Card::new(2, Suit::Spade, InDeck),
            Card::new(3, Suit::Club, InDeck),
            Card::new(3, Suit::Heart, InDeck),
            Card::new(3, Suit::Diamond, InDeck),
            Card::new(3, Suit::Spade, InDeck),
            Card::new(4, Suit::Club, InDeck),
            Card::new(4, Suit::Heart, InDeck),
            Card::new(4, Suit::Diamond, InDeck),
            Card::new(4, Suit::Spade, InDeck),
            Card::new(5, Suit::Club, InDeck),
            Card::new(5, Suit::Heart, InDeck),
            Card::new(5, Suit::Diamond, InDeck),
            Card::new(5, Suit::Spade, InDeck),
            Card::new(6, Suit::Club, InDeck),
            Card::new(6, Suit::Heart, InDeck),
            Card::new(6, Suit::Diamond, InDeck),
            Card::new(6, Suit::Spade, InDeck),
            Card::new(7, Suit::Club, InDeck),
            Card::new(7, Suit::Heart, InDeck),
            Card::new(7, Suit::Diamond, InDeck),
            Card::new(7, Suit::Spade, InDeck),
            Card::new(8, Suit::Club, InDeck),
            Card::new(8, Suit::Heart, InDeck),
            Card::new(8, Suit::Diamond, InDeck),
            Card::new(8, Suit::Spade, InDeck),
            Card::new(9, Suit::Club, InDeck),
            Card::new(9, Suit::Heart, InDeck),
            Card::new(9, Suit::Diamond, InDeck),
            Card::new(9, Suit::Spade, InDeck),
            Card::new(10, Suit::Club, InDeck),
            Card::new(10, Suit::Heart, InDeck),
            Card::new(10, Suit::Diamond, InDeck),
            Card::new(10, Suit::Spade, InDeck),
            Card::new(11, Suit::Club, InDeck),
            Card::new(11, Suit::Heart, InDeck),
            Card::new(11, Suit::Diamond, InDeck),
            Card::new(11, Suit::Spade, InDeck),
            Card::new(12, Suit::Club, InDeck),
            Card::new(12, Suit::Heart, InDeck),
            Card::new(12, Suit::Diamond, InDeck),
            Card::new(12, Suit::Spade, InDeck),
            Card::new(13, Suit::Club, InDeck),
            Card::new(13, Suit::Heart, InDeck),
            Card::new(13, Suit::Diamond, InDeck),
            Card::new(13, Suit::Spade, InDeck),
        ];

        Deck {
            cards
        }
    }
}
