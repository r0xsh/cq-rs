use events::user_events::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub uuid: Option<Uuid>,
    pub deleted: bool,
    pub name: String,
}

impl User {
    pub fn new(stream: &Vec<UserEvent>) -> User {
        let mut user = User::default();

        for event in stream {
            user.apply(event)
        }
        user
    }

    pub fn apply(&mut self, event: &UserEvent) {
        match event {
            UserEvent::Created(e) => {
                self.uuid = Some(e.uuid);
                self.name = e.name.to_owned()
            }
            UserEvent::Deleted => {
                self.deleted = true;
            }
        }
    }

    pub fn default() -> User {
        User {
            uuid: None,
            deleted: false,
            name: String::new(),
        }
    }

    pub fn create(stream: &mut Vec<UserEvent>, uuid: Uuid, name: &str) {
        stream.push(UserEvent::Created(UserEventData {
            uuid: uuid,
            name: name.to_owned(),
        }));
    }

    pub fn delete(&mut self, stream: &mut Vec<UserEvent>) {
        if !self.deleted {
            let event = UserEvent::Deleted;
            self.apply(&event);
            stream.push(event);
        }
    }
}
