#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        //.mount("/", routes![index])
        .mount("/first", routes![first])
        .mount("/world", routes![world])
        .mount("/", FileServer::from("public"))
}