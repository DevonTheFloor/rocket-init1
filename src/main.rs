#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::Responder;
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket::http::{Method, Uri};
use std::path::{Path, PathBuf};

struct CustomFileServer;

impl CustomFileServer {
    fn new() -> FileServer {
        let mut fs = FileServer::from("public");
        fs.options.mime_overrides.push((ContentType::JavaScript, vec![".js"]));
        fs.options.mime_overrides.push((ContentType::CSS, vec![".css"]));
        fs.options.mime_overrides.push((ContentType::HTML, vec![".html"]));
        fs
    }
}

impl Responder<'_> for CustomFileServer {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'static> {
        self.new().respond_to(Request::new(&Method::Get, Uri::from_static("/")))
    }
}

#[get("/first")]
fn first() -> &'static str {
    "C'est le premier !!!"
}

#[get("/world")]
fn world() -> &'static str {
    "hello, world!"
}

#[get("/statics/<file_path..>")]
fn statics(file_path: PathBuf) -> Option<NamedFile> {
    CustomFileServer.respond_to(Request::new(&Method::Get, Uri::from_static("/")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/first", routes![first])
        .mount("/world", routes![world])
        .mount("/", routes![statics])
}


