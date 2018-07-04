extern crate uuid;

mod dto;
mod events;

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use dto::user_dto::*;
    use events::user_events::*;

    #[test]
    fn raise_user_created_when_create_user() {
        let uuid = Uuid::new_v4();
        let mut stream = vec![];
        User::create(&mut stream, uuid, "Antoine");

        assert_eq!(
            stream[0],
            UserEvent::Created(UserEventData {
                uuid: uuid,
                name: "Antoine".to_string()
            })
        )
    }

    #[test]
    fn raise_user_deleted_when_delete_user() {
        let mut stream = vec![];
        User::create(&mut stream, Uuid::new_v4(), "Antoine");

        let mut user = User::new(&stream);
        user.delete(&mut stream);

        assert_eq!(*stream.last().unwrap(), UserEvent::Deleted);
        assert_eq!(stream.len(), 2);
    }

    #[test]
    fn not_raise_user_deleted_when_delete_deleted_user() {
        let mut stream = vec![];
        User::create(&mut stream, Uuid::new_v4(), "Antoine");

        let mut user = User::new(&stream);
        user.delete(&mut stream);
        user.delete(&mut stream);

        assert_eq!(*stream.last().unwrap(), UserEvent::Deleted);
        assert_eq!(stream.len(), 2);
    }

}
