use texas_holdem::card::{Card, State};
use texas_holdem::hand::HAND_NUM;
use texas_holdem::player::Player;
use texas_holdem::suit::Suit;

#[test]
fn it_works() {
    #[derive(Debug)]
    struct TestCase {
        name: String,
        args: [Card; HAND_NUM],
        expected: Player,
    }

    let table = [
        TestCase {
            args: [Card::new(1, Suit::Club, State::InHand), Card::new(1, Suit::Club, State::InHand)],
            expected: Player {
                cards: [Card::new(1, Suit::Club, State::InHand), Card::new(1, Suit::Club, State::InHand)],
                chip: 0,
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