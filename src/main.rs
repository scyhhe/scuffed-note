use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Mutex,
};

use model::{Note, NoteDto};

use rand::{distributions::Alphanumeric, *};

use rocket::{
    http::Status,
    serde::json::{serde_json::json, Json, Value},
    Request, State,
};

#[macro_use]
extern crate rocket;

mod model;

struct Persistence {
    storage: HashMap<String, Note>,
}
impl Persistence {
    pub fn use_note(&mut self, hash: &String) -> Option<Note> {
        let note = self.storage.entry(hash.clone());
        match note {
            Entry::Vacant(_) => None,
            Entry::Occupied(o) if o.get().uses > 1 => {
                let mut mutable_note = o.into_mut();
                mutable_note.uses -= 1;
                Some(mutable_note.clone())
            }
            Entry::Occupied(o) => Some(o.remove_entry().1),
        }
    }
}

#[get("/notes/<hash>", format = "json")]
fn fetch_note(db: &State<Mutex<Persistence>>, hash: &str) -> (Status, Option<Value>) {
    let lookup = db
        .lock()
        .ok()
        .and_then(|mut notes| notes.use_note(&hash.to_string()));

    let note = lookup.map(|note| {
        json!({
            "content": note.contents
        })
    });

    match note {
        Some(_) => (Status::Ok, note),
        None => (Status::NotFound, None),
    }
}

#[post("/notes", format = "json", data = "<content>")]
fn create_note(db: &State<Mutex<Persistence>>, content: Json<NoteDto>) -> (Status, Value) {
    let hash_value = rand();
    let note = Note::from(content.into_inner());

    db.lock()
        .ok()
        .and_then(|mut notes| notes.storage.insert(hash_value.clone(), note));

    (Status::Created, json!({ "url": build_url(&hash_value) }))
}

#[catch(404)]
fn not_found(req: &Request) -> (Status, Value) {
    (
        Status::NotFound,
        json!(
            {
                "message": format!("nothing found under {}", req.uri()),
                "code": "404"
            }
        ),
    )
}

fn build_url(hash: &String) -> String {
    format!("http://localhost:8000/notes/{}", hash)
}

fn rand() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

// fn encrypt(contents: &String) -> String {
//     let cipher = ShortCrypt::new(rand());
//     cipher.encrypt_to_url_component(contents)
// }

#[launch]
fn rocket() -> _ {
    let state = Mutex::new(Persistence {
        storage: HashMap::new(),
    });

    rocket::build()
        .manage(state)
        .mount("/", routes![fetch_note, create_note])
        .register("/", catchers![not_found])
}
