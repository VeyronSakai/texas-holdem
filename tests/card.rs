use texas_holdem::card::Card;
use texas_holdem::suit::Suit;

#[test]
fn it_works() {
    #[derive(Debug)]
    struct TestCase {
        args: (u8, Suit),
        expected: Card,
        name: String,
    }

    let table = [
        TestCase {
            args: (1, Suit::Club),
            expected: Card {
                number: 1,
                suit: Suit::Club,
            },
            name: String::from("正常系1"),
        },
        TestCase {
            args: (2, Suit::Heart),
            expected: Card {
                number: 2,
                suit: Suit::Heart,
            },
            name: String::from("正常系2"),
        },
    ];

    for test_case in table {
        assert_eq!(
            Card::new(test_case.args.0, test_case.args.1),
            test_case.expected,
            "Failed in the {:?}.",
            test_case,
        );
    }
}

#[test]
#[should_panic]
fn it_doesnt_work() {
    Card::new(0, Suit::Club);
    Card::new(20, Suit::Club);
}