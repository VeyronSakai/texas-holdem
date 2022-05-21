use crate::domain::player::player_id::PlayerId;
use crate::domain::player::player_repository::PlayerRepository;
use crate::domain::player::Player;
use std::collections::HashMap;

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
        todo!()
    }
}
