// https://imfeld.dev/writing/actix-web-middleware
// curl -XGET -H'x-test-header: headervalue' localhost:8080/hello/asdf
mod middleware;
use middleware::*;

use actix_web::{get, web, App, HttpServer, Responder, Either, HttpResponse, Error};
use actix_web::http::{Method, StatusCode};
use actix_files::{Files, NamedFile};
use actix_web::dev::Service;
use actix_web::web::service;

// #[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!\n")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .wrap(middleware::api_key::ApiKeyMiddlware)
            .wrap(middleware::api_key::ApiKey::new("asdf".to_string()))
            // .service(root)
            // .service()
            .default_service(web::to(default_handler))
            .service(web::resource("/hello/{name}").to(greet))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


async fn default_handler(req_method: Method) -> Result<impl Responder, Error> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}
