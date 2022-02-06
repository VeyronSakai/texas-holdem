use texas_holdem::card::Card;
use texas_holdem::card::State::{self, InDeck, InHand};
use texas_holdem::suit::Suit;

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

#[test]
#[should_panic]
fn it_doesnt_work() {
    Card::new(0, Suit::Club, InHand);
    Card::new(20, Suit::Club, InHand);
}