use crate::suit::Suit;

#[derive(PartialEq, Debug)]
struct Card {
    number: u8,
    suit: Suit,
}

impl Card {
    fn new(number: u8, suit: Suit) -> Card {
        if number < 1 || 13 < number {
            panic!("トランプの数字は1以上13以下でなければなりません。")
        }

        Card { number, suit }
    }
}

#[test]
fn new_test() {
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
fn new_test3() {
    Card::new(0, Suit::Club);
    Card::new(20, Suit::Club);
}
