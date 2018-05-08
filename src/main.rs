extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix::prelude::*;
use actix_web::{http, server, App, HttpRequest, HttpResponse};

#[derive(Serialize, Deserialize)]
pub struct Message<'a> {
    pub message: &'a str,
}

fn json(req: HttpRequest) -> HttpResponse {
    let message = Message {
        message: "Hello, World!",
    };
    let json = serde_json::to_string(&message).unwrap();

    HttpResponse::build_from(&req)
        .header(http::header::SERVER, "Actix")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(json)
}

fn main() {
    let sys = System::new("jake");

    // start http server
    server::new(|| {
        App::new()
            .resource("/json", |r| r.f(json))
    }).bind("0.0.0.0:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}