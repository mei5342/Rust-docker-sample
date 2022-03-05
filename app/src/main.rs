// use actix_web::{get, web, App, HttpServer, Responder};

// #[get("/{id}/{name}/index.html")]
// async fn index(params: web::Path<(u32, String)>) -> impl Responder {
//     let (id, name) = params.into_inner();
//     format!("Hello {}! id:{}", name, id)
// }

// #[actix_web::main] // or #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}