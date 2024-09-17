use actix_web::{web, App, HttpServer}; // import components from actix-web
#[path = "routes/index.rs"] mod index; // how to import a file from another folder

#[actix_web::main] // creates entry point for http server
pub async fn main() -> std::io::Result<()> // Result may fail
{
    HttpServer::new(|| { // creates a new http server
        App::new() // creates a new web application
        .route("/", web::get().to(index::index)) // address , segway to index
    })
    .bind("127.0.0.1:8080")? // root address
    .run()
    .await // its an async func
}


