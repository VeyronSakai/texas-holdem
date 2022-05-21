use crate::domain::player::player_id::PlayerId;
use crate::domain::player::player_repository::PlayerRepository;
use crate::domain::player::Player;
use std::collections::HashMap;
use crate::domain::card::{Card, State};
use crate::domain::suit::Suit;

struct PlayerRepositoryImpl {
    db: HashMap<PlayerId, Player>,
}

impl PlayerRepositoryImpl {
    fn new() -> Self {
        PlayerRepositoryImpl { db: HashMap::new() }
    }
}

impl PlayerRepository for PlayerRepositoryImpl {
    fn find(&self, id: PlayerId) -> &Player {
        self.db.get(&id).unwrap()
    }

    fn save(&mut self, player: Player) {
        self.db.insert(player.id, player);
    }
}

#[test]
fn find_test() {
    let player_id = PlayerId::new(uuid::Uuid::default());

    let player = Player::new(
        player_id,
        [Card::new(1, Suit::Club, State::InHand), Card::new(2, Suit::Club, State::InHand)],
        0,
    );

    let mut mp: HashMap<PlayerId, Player> = HashMap::new();
    mp.insert(player_id, player.clone());

    let repo = PlayerRepositoryImpl {
        db: mp
    };

    assert_eq!(*repo.find(player_id), player);
}
