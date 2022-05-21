use crate::domain::card::{Card, State};
use crate::domain::card::State::{InDeck, InHand};
use crate::domain::deck::Deck;
use crate::domain::player::Player;
use crate::domain::player::player_id::PlayerId;
use crate::domain::suit::Suit;

#[derive(PartialEq, Debug)]
pub struct Dealer {
    pub deck: Deck,
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer {
            deck: Deck::new()
        }
    }

    // Shuffle deck
    fn shuffle(&mut self) {
        self.deck.shuffle_cards();
    }

    fn provide_cards(&mut self, players: Vec<&mut Player>) {
        for player in players {
            let mut card1 = self.deck.cards.pop().unwrap();
            card1.state = State::InHand;
            let mut card2 = self.deck.cards.pop().unwrap();
            card2.state = State::InHand;
            player.cards.push(card1);
            player.cards.push(card2);
        }
    }
}

#[test]
fn new_test() {
    use crate::domain::deck::tests::build_mock_deck;

    let want = Dealer {
        deck: build_mock_deck()
    };

    assert_eq!(Dealer::new(), want);
}

#[test]
fn provide_cards_test() {
    use crate::domain::deck::tests::build_mock_deck;

    let mut dealer = Dealer {
        deck: build_mock_deck()
    };

    let player1_id = PlayerId::new(uuid::Uuid::new_v4());
    let mut player1 = Player::new(player1_id, 0);

    let player2_id = PlayerId::new(uuid::Uuid::new_v4());
    let mut player2 = Player::new(player2_id, 0);

    dealer.provide_cards(vec![&mut player1, &mut player2]);

    assert_eq!(player1.cards, vec![Card::new(13, Suit::Spade, InHand), Card::new(13, Suit::Diamond, InHand)]);
    assert_eq!(player2.cards, vec![Card::new(13, Suit::Heart, InHand), Card::new(13, Suit::Club, InHand)]);

    let cards = vec![
        Card::new(1, Suit::Club, InDeck),
        Card::new(1, Suit::Heart, InDeck),
        Card::new(1, Suit::Diamond, InDeck),
        Card::new(1, Suit::Spade, InDeck),
        Card::new(2, Suit::Club, InDeck),
        Card::new(2, Suit::Heart, InDeck),
        Card::new(2, Suit::Diamond, InDeck),
        Card::new(2, Suit::Spade, InDeck),
        Card::new(3, Suit::Club, InDeck),
        Card::new(3, Suit::Heart, InDeck),
        Card::new(3, Suit::Diamond, InDeck),
        Card::new(3, Suit::Spade, InDeck),
        Card::new(4, Suit::Club, InDeck),
        Card::new(4, Suit::Heart, InDeck),
        Card::new(4, Suit::Diamond, InDeck),
        Card::new(4, Suit::Spade, InDeck),
        Card::new(5, Suit::Club, InDeck),
        Card::new(5, Suit::Heart, InDeck),
        Card::new(5, Suit::Diamond, InDeck),
        Card::new(5, Suit::Spade, InDeck),
        Card::new(6, Suit::Club, InDeck),
        Card::new(6, Suit::Heart, InDeck),
        Card::new(6, Suit::Diamond, InDeck),
        Card::new(6, Suit::Spade, InDeck),
        Card::new(7, Suit::Club, InDeck),
        Card::new(7, Suit::Heart, InDeck),
        Card::new(7, Suit::Diamond, InDeck),
        Card::new(7, Suit::Spade, InDeck),
        Card::new(8, Suit::Club, InDeck),
        Card::new(8, Suit::Heart, InDeck),
        Card::new(8, Suit::Diamond, InDeck),
        Card::new(8, Suit::Spade, InDeck),
        Card::new(9, Suit::Club, InDeck),
        Card::new(9, Suit::Heart, InDeck),
        Card::new(9, Suit::Diamond, InDeck),
        Card::new(9, Suit::Spade, InDeck),
        Card::new(10, Suit::Club, InDeck),
        Card::new(10, Suit::Heart, InDeck),
        Card::new(10, Suit::Diamond, InDeck),
        Card::new(10, Suit::Spade, InDeck),
        Card::new(11, Suit::Club, InDeck),
        Card::new(11, Suit::Heart, InDeck),
        Card::new(11, Suit::Diamond, InDeck),
        Card::new(11, Suit::Spade, InDeck),
        Card::new(12, Suit::Club, InDeck),
        Card::new(12, Suit::Heart, InDeck),
        Card::new(12, Suit::Diamond, InDeck),
        Card::new(12, Suit::Spade, InDeck)];
    assert_eq!(dealer.deck.cards, cards)
}
