use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;

use serde::Serialize;

#[derive(Serialize)]
struct AtCoder {
    contests: Vec<String>,
}

#[get("/api/v1/atcoder")]
async fn atcoder() -> impl Responder {
    let mut manifest_dir: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    manifest_dir.pop();
    manifest_dir.push("atcoder");

    let atcoder_list: Vec<String> = read_dir(&manifest_dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();

    HttpResponse::Ok().json(AtCoder {
        contests: atcoder_list,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(atcoder))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
