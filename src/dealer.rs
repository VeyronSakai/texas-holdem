use crate::card::Card;
use crate::card::State::InDeck;
use crate::suit::Suit;

#[derive(PartialEq, Debug)]
pub struct Dealer {
    pub deck: Vec<Card>,
}

impl Dealer {
    // pub fn new() -> Dealer {
    //     let mut deck: Vec<Card> = Vec::new();
    //     for num in 1..=13 {
    //         deck.push(Card::new(num, Suit::Club, InDeck));
    //         deck.push(Card::new(num, Suit::Heart, InDeck));
    //         deck.push(Card::new(num, Suit::Diamond, InDeck));
    //         deck.push(Card::new(num, Suit::Spade, InDeck));
    //     }
    //
    //     Dealer {
    //         deck
    //     }
    // }

    // Shuffle deck
    fn shuffle(&mut self) {
        self.deck[0] = Card::new(1, Suit::Heart, InDeck);
    }
}



// #[test]
// fn shuffle_test() {
//     use crate::dealer::tests::build_mock_deck;
//
//     #[derive(Debug)]
//     struct TestCase {
//         name: String,
//         expected: Dealer,
//     }
//
//     let mut dealer = Dealer::new();
//
//     dealer.shuffle();
//
//     println!("{:?}", dealer.deck)
// }
//
