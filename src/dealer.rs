use crate::card::Card;
use crate::card::State::InDeck;
use crate::suit::Suit;

#[derive(PartialEq, Debug)]
struct Dealer {
    deck: Vec<Card>,
}

impl Dealer {
    fn new() -> Dealer {
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

// #[test]
// fn it_works() {
//     use super::*;
//
//     #[derive(Debug)]
//     struct TestCase {
//         name: String,
//         expected: Dealer,
//     }
//
//     let table = [
//         TestCase {
//             expected: Dealer {
//                 deck: vec![],
//             },
//             name: String::from("正常系1"),
//         },
//     ];
//
//     for test_case in table {
//         assert_eq!(
//             Dealer::new(),
//             test_case.expected,
//             "Failed in the {:?}.",
//             test_case,
//         );
//     }
// }