use events::user_events::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub uuid: Option<Uuid>,

    #[serde(skip_deserializing)]
    pub deleted: bool,

    pub name: String,
}

impl<'a> From<&'a Vec<UserEvent>> for User {
    fn from(stream: &Vec<UserEvent>) -> Self {
        let mut user = User::default();

        for event in stream {
            user.apply(event)
        }
        user
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            uuid: None,
            deleted: false,
            name: String::new(),
        }
    }
}

impl User {
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
