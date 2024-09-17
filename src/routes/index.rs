use actix_web::{HttpResponse, Responder}; // import components from actix-web

// the index handler function
pub async fn index() -> impl Responder // reutns an http response
{
    HttpResponse::Ok().body("Hello, Crissy!") // http response
}
