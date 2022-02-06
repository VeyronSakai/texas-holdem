use texas_holdem::board::Board;
use texas_holdem::card::Card;
use texas_holdem::card::State::{self, InDeck, InHand};
use texas_holdem::suit::Suit;

#[test]
fn it_works() {
    #[derive(Debug)]
    struct TestCase {
        args: Vec<Card>,
        expected: Board,
        name: String,
    }

    let table = [
        TestCase {
            args: vec![Card::new(1, Suit::Club, State::InHand)],
            expected: Board {
                cards: vec![Card::new(1, Suit::Club, State::InHand)]
            },
            name: String::from("正常系1"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Board::new(test_case.args.clone()),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}