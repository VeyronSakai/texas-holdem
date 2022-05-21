use uuid::Uuid;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PlayerId {
    pub value: Uuid,
}

impl PlayerId {
    pub fn new(uuid: Uuid) -> PlayerId {
        PlayerId {
            value: uuid
        }
    }
}