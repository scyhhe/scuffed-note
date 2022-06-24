# Scuffed Note

An [Ali Express](https://aliexpress.com/) version of [privnote](https://privnote.com)

## Goals

* Experiment with [rocket](https://rocket.rs)
* Learn some rust
* Have some fun
* Create a simple web service that can:
  * create private notes with limited uses
  * fetch notes via url

## Non Goals

* Deployment on actual prod environment

## TODOS

* [ ] Encrypt notes on creation
* [ ] Password protect notes
* [ ] Add auto-expire to notes
* [x] Add usage counter to notes
* [x] Add unique urls to notes

## How to run

1. Install rust. Easily done via [rustup](https://rustup.rs/)
2. Clone Project
3. cd scuffed-note && cargo run
4. should get the `🚀 Rocket has launched from http://127.0.0.1:8000`

To create notes, fire up a POST request to `localhost:8000/notes` with the following JSON body:

```json
{
    "contents": "my super secret note",
    "uses": 1
}
```

Which will give you back a JSON response with the URL at which you can fetch it:

```json
{
    "url": "http://localhost:8000/notes/uDxj46z2AhON9ev2"
}
```

## Contributors

* [Vasil Statev](https://github.com/VStatev)
* [Michele Vigilante](https://github.com/zZKato)
