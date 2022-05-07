use crate::card::Card;
use crate::card::State::InDeck;
use crate::deck::Deck;
use crate::suit::Suit;

#[derive(PartialEq, Debug)]
pub struct Dealer {
    pub deck: Deck,
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer {
            deck: Deck::new()
        }
    }

    // Shuffle deck
    fn shuffle(&mut self) {
        self.deck.shuffle_cards();
    }
}

#[test]
fn new_test() {
    use crate::deck::tests::build_mock_deck;

    let want = Dealer {
        deck: build_mock_deck()
    };

    assert_eq!(Dealer::new(), want);
}
