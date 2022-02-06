use texas_holdem::card::Card;
use texas_holdem::hand::Hand;
use texas_holdem::suit::Suit;

#[test]
fn it_works() {
    #[derive(Debug)]
    struct TestCase {
        args: (Card, Card),
        expected: Hand,
        name: String,
    }

    let table = [
        TestCase {
            args: (Card::new(1, Suit::Club), Card::new(1, Suit::Heart)),
            expected: Hand {
                card1: Card::new(1, Suit::Club),
                card2: Card::new(1, Suit::Heart),
            },
            name: String::from("正常系1"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Hand::new(test_case.args.0, test_case.args.1),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}

#[test]
#[should_panic]
fn it_doesnt_work() {
    Hand::new(Card::new(5, Suit::Club), Card::new(5, Suit::Club));
}