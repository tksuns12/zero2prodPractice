use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await

}
