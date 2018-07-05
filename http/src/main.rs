extern crate actix_web;
extern crate domain;
extern crate uuid;
#[macro_use]
extern crate lazy_static;

use uuid::Uuid;

use domain::dto::user_dto::*;
use domain::events::user_events::*;

use actix_web::{http, server, App, HttpRequest, Json, Responder};

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref STREAM: Mutex<HashMap<String, Vec<UserEvent>>> = Mutex::new(HashMap::new());
}

fn create(info: Json<User>) -> impl Responder {
    let uuid = Uuid::new_v4();
    let event = UserEvent::Created(UserEventData {
        uuid: uuid,
        name: info.name.to_owned(),
    });
    let response = format!("{} => {:?}", uuid, event);

    STREAM.lock().unwrap().insert(uuid.to_string(), vec![event]);

    response
}

fn delete(req: HttpRequest) -> impl Responder {
    let ref uuid: String = req.match_info().query("id").unwrap();

    STREAM
        .lock()
        .unwrap()
        .get_mut(uuid)
        .unwrap()
        .push(UserEvent::Deleted);

    format!("Ok {}", uuid)
}

fn index(req: HttpRequest) -> impl Responder {
    let ref uuid: String = req.match_info().query("id").unwrap();

    let streams = STREAM.lock().unwrap();
    let stream = streams.get(uuid).unwrap();
    let user = User::from(stream);

    format!("{:?}", user)
}

fn main() {
    server::new(|| {
        App::new()
            .route("/", http::Method::POST, create)
            .route("/{id}", http::Method::DELETE, delete)
            .route("/{id}", http::Method::GET, index)
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
