#[macro_use]
extern crate rocket;

use rand::seq::IteratorRandom;
use rocket::fs::{FileServer, NamedFile};
use std::fs;

#[get("/tunes")]
async fn tunes() -> Option<NamedFile> {
    NamedFile::open(randomfile("./tunes")).await.ok()
}

#[get("/clock")]
async fn clock() -> Option<NamedFile> {
    NamedFile::open("static/clock.html").await.ok()
}

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open("static/404.html").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![clock, tunes])
        .mount("/", FileServer::from("static/").rank(1))
        .mount("/share", FileServer::from("share/").rank(2))
}

fn randomfile(path: &str) -> String {
    let mut rng = rand::thread_rng();
    let files = fs::read_dir(path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    file.path().display().to_string()
}
