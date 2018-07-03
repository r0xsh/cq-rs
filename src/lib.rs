pub trait Event {}
pub struct Message;

#[derive(Debug)]
pub struct MessageEventData {
    pub message: String
}

#[derive(Debug)]
pub enum MessageEvent {
    Quacked(MessageEventData)
}

impl PartialEq for MessageEvent {
    fn eq(&self, other: &MessageEvent) -> bool {
        match (self, other) {
            (MessageEvent::Quacked(a), MessageEvent::Quacked(b)) => {
                return a == b;
            }
        }
    }
}


impl PartialEq for MessageEventData {
    fn eq(&self, other: &MessageEventData) -> bool {
        self.message == other.message
    }
}

impl Message {
    pub fn quack(message: &str) -> MessageEvent {
        MessageEvent::Quacked(MessageEventData {
            message: message.to_owned()
        })
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raise_message() {
        let mut history = vec!();
        history.push(Message::quack("coucou"));

        assert_eq!(
            history[0],
            MessageEvent::Quacked(MessageEventData{
                message: "coucou".to_string()
            })
        )
    }
}
