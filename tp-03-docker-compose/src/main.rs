use std::{collections::HashMap, env};
use actix_web::{get, App, HttpResponse, HttpServer, http::{header::{HeaderMap}, StatusCode}, HttpRequest, Responder, web::{self}};

#[get("/ping")]
async fn ping(req: HttpRequest) -> impl Responder {
    println!("{}", req.connection_info().host());
    HttpResponse::Ok().json(convert(req.headers()))
}

fn convert(headers: &HeaderMap) -> HashMap<String, Vec<String>> {
    let mut header_hashmap = HashMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(Vec::new).push(v)
    }
    header_hashmap
}

async fn not_found() -> impl Responder {
    HttpResponse::build(StatusCode::NOT_FOUND).body("404")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key: &str = "PING_LISTEN_PORT";
    let port = match env::var(key) {
        Ok(val) => val,
        Err(_) => 8080.to_string(),
    };
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .default_service(web::route().to(not_found))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}