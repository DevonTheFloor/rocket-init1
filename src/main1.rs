#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket::http::Status;
use std::path::Path;
/*#[get("/")]
fn index() -> &'static str {
    "Comment ça va"
}*/

#[get("/first")]
fn first() -> &'static str {
    "C'est le premier !!!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}
#[get("/statics/<file_path>")]
fn statics(file_path: &str) -> Option<&str> {
    Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        //.mount("/", routes![index])
        .mount("/first", routes![first])
        .mount("/world", routes![world])
        .mount("/", FileServer::from("public"))
}