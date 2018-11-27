extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;

use actix_web::{http, middleware, server, App, Form, HttpRequest, Json, Query, Result};

#[derive(Deserialize, Serialize)]
struct Info {
    username: String,
}

fn index(info: Form<Info>) -> Result<Json<Info>> {
    Ok(Json(Info {
        username: info.username.clone() + "a",
    }))
}

fn main() {
    let sys = actix::System::new("hello-world");

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::POST).with(index))
    })
    .bind("127.0.0.1:12345")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:12345");
    let _ = sys.run();
}
