extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix::prelude::*;
use actix_web::{server, App, HttpRequest, HttpResponse, Json};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
}

fn list(req: HttpRequest) -> HttpResponse {
    HttpResponse::build_from(&req).json(json!({"Hello": "world!"}))
}

fn create(req: HttpRequest, body: Json<User>) -> HttpResponse {
    let user = body.into_inner();
    HttpResponse::build_from(&req).json(user)
}

fn main() {
    let sys = System::new("jake");

    // start http server
    server::new(|| {
        App::new()
            .resource("/users", |r| {
                r.get().with(list);
                r.post().with2(create);
            })
    }).bind("0.0.0.0:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}