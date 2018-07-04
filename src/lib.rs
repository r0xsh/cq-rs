extern crate uuid;
use uuid::Uuid;

pub trait Event {}

#[derive(Debug)]
pub struct Message {
    pub uuid: Option<Uuid>,
    pub deleted: bool,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub struct MessageEventData {
    pub uuid: Uuid,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum MessageEvent {
    Quacked(MessageEventData),
    Deleted,
}

impl Message {
    pub fn new(events: &Vec<MessageEvent>) -> Message {
        let mut msg = Message::default();

        for event in events {
            msg.apply(event)
        }
        msg
    }

    pub fn apply(&mut self, event: &MessageEvent) {
        match event {
            MessageEvent::Quacked(e) => {
                self.uuid = Some(e.uuid);
                self.message = e.message.to_owned()
            }
            MessageEvent::Deleted => {
                self.deleted = true;
            }
        }
    }

    pub fn default() -> Message {
        Message {
            uuid: None,
            deleted: false,
            message: String::new(),
        }
    }

    pub fn quack(events: &mut Vec<MessageEvent>, uuid: Uuid, message: &str) {
        events.push(MessageEvent::Quacked(MessageEventData {
            uuid: uuid,
            message: message.to_owned(),
        }));
    }

    pub fn delete(&mut self, events: &mut Vec<MessageEvent>) {
        if !self.deleted {
            let event = MessageEvent::Deleted;
            self.apply(&event);
            events.push(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raise_message_quacked_when_quack_message() {
        let uuid = Uuid::new_v4();
        let mut history = vec![];
        Message::quack(&mut history, uuid, "coucou");

        assert_eq!(
            history[0],
            MessageEvent::Quacked(MessageEventData {
                uuid: uuid,
                message: "coucou".to_string()
            })
        )
    }

    #[test]
    fn raise_message_deleted_when_delete_message() {
        let mut history = vec![];
        Message::quack(&mut history, Uuid::new_v4(), "coucou");

        let mut message = Message::new(&history);
        message.delete(&mut history);

        assert_eq!(*history.last().unwrap(), MessageEvent::Deleted);
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn not_raise_message_deleted_when_delete_deleted_message() {
        let mut history = vec![];
        Message::quack(&mut history, Uuid::new_v4(), "coucou");

        let mut message = Message::new(&history);
        message.delete(&mut history);
        message.delete(&mut history);

        assert_eq!(history.len(), 2);
    }

}
