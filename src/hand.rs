use crate::card::{Card, State};

pub const HAND_NUM: usize = 2;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Hand {
    pub cards: [Card; HAND_NUM],
}

impl Hand {
    pub fn new(cards: [Card; HAND_NUM]) -> Hand {
        if cards[0] == cards[1] {
            panic!("The exact same card will not be dealt.")
        }

        Hand { cards }
    }
}

#[test]
fn it_works() {
    use crate::suit::Suit;
    use crate::card::State;

    #[derive(Debug)]
    struct TestCase {
        args: [Card; HAND_NUM],
        expected: Hand,
        name: String,
    }

    let table = [
        TestCase {
            args: [Card::new(1, Suit::Club, State::InDeck), Card::new(1, Suit::Heart, State::InDeck)],
            expected: Hand {
                cards: [Card::new(1, Suit::Club, State::InDeck), Card::new(1, Suit::Heart, State::InDeck)],
            },
            name: String::from("正常系1"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Hand::new(test_case.args),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}
