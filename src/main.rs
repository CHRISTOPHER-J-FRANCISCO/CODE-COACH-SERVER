use actix_web::{get, web, App, HttpResponse, HttpServer, Responder}; // import components from actix-web

#[actix_web::main] // creates entry point for http server
async fn main() -> std::io::Result<()> // Result may fail
{
    HttpServer::new(|| { // creates a new http server
        App::new() // creates a new web application
        .route("/", web::get().to(index)) // address , segway to index
    })
    .bind("127.0.0.1:8080")? // root address
    .run()
    .await // its an async func
}

// the index handler function
async fn index() -> impl Responder // reutns an http response
{
    HttpResponse::Ok().body("Hello, Crissy!") // http response
}

