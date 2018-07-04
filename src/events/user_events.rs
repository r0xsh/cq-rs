use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct UserEventData {
    pub uuid: Uuid,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum UserEvent {
    Created(UserEventData),
    Deleted,
}
