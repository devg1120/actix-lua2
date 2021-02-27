extern crate actix;
extern crate actix_lua;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use actix::prelude::*;
use actix_lua::{LuaActor, LuaActorBuilder, LuaMessage};
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, State,
};
use futures::Future;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct AppState {
    lua: Addr<LuaActor>,
}

fn build_message(path: Path<String>, method: &str) -> LuaMessage {
    let mut t = HashMap::new();
    t.insert("path".to_string(), LuaMessage::from(path.into_inner()));
    t.insert("method".to_string(), LuaMessage::from(method.to_string()));

    LuaMessage::from(t)
}

fn run_lua(state: State<AppState>, t: LuaMessage) -> FutureResponse<HttpResponse> {
    state
        .lua
        .send(LuaMessage::from(t))
        .from_err()
        .and_then(|res| {
            match res {
                LuaMessage::String(s) => Ok(HttpResponse::Ok().body(s)),

                // ignore everything else
                _ => unimplemented!(),
            }
        })
        .responder()
}

fn get((path, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    run_lua(state, build_message(path, "GET"))
}

fn post((path, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    run_lua(state, build_message(path, "POST"))
}

fn put((path, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    run_lua(state, build_message(path, "PUT"))
}

fn delete((path, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    run_lua(state, build_message(path, "DELETE"))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("lua-web");

    let addr = Arbiter::start(|_| {
        let script = read_to_string(&::std::env::args().nth(1).unwrap());

        let lua = LuaActorBuilder::new()
            .on_handle_with_lua(&script)
            .build()
            .unwrap();

        lua
    });

    // Start http server
    server::new(move || {
        App::with_state(AppState{lua: addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/{path:.*}", |r| {
                r.method(http::Method::GET).with(get);
                r.method(http::Method::POST).with(post);
                r.method(http::Method::PUT).with(put);
                r.method(http::Method::DELETE).with(delete)
            })
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}

fn read_to_string(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut body = String::new();
    f.read_to_string(&mut body).expect("Failed to read file");

    body
}
