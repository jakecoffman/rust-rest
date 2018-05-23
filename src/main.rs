extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate env_logger;

use actix::prelude::*;
use actix_web::{middleware, server, App, HttpResponse};
use actix_web::dev::ResourceHandler;
use std::cell::Cell;

mod users;

pub struct AppState {
    pub counter: Cell<usize>,
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = System::new("jake");

    // start http server
    server::new(|| {
        App::with_state(AppState{counter: Cell::new(0)})
            .middleware(middleware::Logger::default())
            .resource("/", index)
            .resource("/users", users::routes)
    }).bind("0.0.0.0:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}

fn index(r: &mut ResourceHandler<AppState>) {
    r.get().f(|_| HttpResponse::Found().header("Location", "/users").finish());
}
