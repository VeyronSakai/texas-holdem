use crate::domain::deck::Deck;

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
    use crate::domain::deck::tests::build_mock_deck;

    let want = Dealer {
        deck: build_mock_deck()
    };

    assert_eq!(Dealer::new(), want);
}
