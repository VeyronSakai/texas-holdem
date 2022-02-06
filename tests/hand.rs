use texas_holdem::card::{Card, State};
use texas_holdem::hand::{Hand, HAND_NUM};
use texas_holdem::suit::Suit;

#[test]
fn it_works() {
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

#[test]
#[should_panic]
fn it_doesnt_work() {
    Hand::new([Card::new(5, Suit::Club, State::InHand), Card::new(5, Suit::Club, State::InHand)]);
}