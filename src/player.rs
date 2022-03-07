use crate::card::{Card, State};
use crate::suit::Suit;

const HAND_NUM: usize = 2;

#[derive(PartialEq, Debug)]
pub struct Player {
    pub cards: [Card; HAND_NUM],
}

impl Player {
    pub fn new(cards: [Card; HAND_NUM]) -> Player {
        Player {
            cards
        }
    }
}

#[test]
fn it_works() {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        args: [Card; HAND_NUM],
        expected: Player,
        name: String,
    }

    let table = [
        TestCase {
            args: [Card::new(1, Suit::Club, State::InHand), Card::new(1, Suit::Club, State::InHand)],
            expected: Player {
                cards: [Card::new(1, Suit::Club, State::InHand), Card::new(1, Suit::Club, State::InHand)]
            },
            name: String::from("正常系1"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Player::new(test_case.args),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}