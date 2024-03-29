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
fn it_works() {
    let player_id = PlayerId::new(uuid::Uuid::default());

    let player = Player::new(
        player_id,
        0,
    );

    let mut repo = PlayerRepositoryImpl::new();
    repo.save(player.clone());

    assert_eq!(*repo.find(player_id), player);
}
