use crate::domain::card::State::{InDeck, InHand};
use crate::domain::suit::Suit;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Card {
    pub number: u8,
    pub suit: Suit,
    pub state: State,
}

impl Card {
    pub fn new(number: u8, suit: Suit, state: State) -> Card {
        if number < 1 || 13 < number {
            panic!("The number of playing cards must be between 1 and 13.")
        }

        Card { number, suit, state }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum State {
    InDeck,

    InHand,

    OnBoard,
}


#[test]
fn it_works() {

    #[derive(Debug)]
    struct TestCase {
        args: (u8, Suit, State),
        expected: Card,
        name: String,
    }

    let table = [
        TestCase {
            args: (1, Suit::Club, State::InDeck),
            expected: Card {
                number: 1,
                suit: Suit::Club,
                state: InDeck,
            },
            name: String::from("正常系1"),
        },
        TestCase {
            args: (2, Suit::Heart, State::InHand),
            expected: Card {
                number: 2,
                suit: Suit::Heart,
                state: InHand,
            },
            name: String::from("正常系2"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Card::new(test_case.args.0, test_case.args.1, test_case.args.2),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}
