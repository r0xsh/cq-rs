pub trait Event {}
pub struct Message;

#[derive(Debug, PartialEq)]
pub struct MessageEventData {
    pub message: String
}

#[derive(Debug, PartialEq)]
pub enum MessageEvent {
    Quacked(MessageEventData)
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
        history.push(Message::quack("coudcou"));

        assert_eq!(
            history[0],
            MessageEvent::Quacked(MessageEventData{
                message: "coucou".to_string()
            })
        )
    }
}
