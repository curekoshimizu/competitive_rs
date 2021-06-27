use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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

    let mut atcoder_list: Vec<String> = read_dir(&manifest_dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect::<Vec<String>>();
    atcoder_list.sort();

    HttpResponse::Ok().json(AtCoder {
        contests: atcoder_list,
    })
}

#[derive(Serialize)]
struct AtCoderEntry {
    name: char,
    solved: bool,
    keywords: Vec<String>,
}

#[get("/api/v1/atcoder/{contest}")]
async fn atcoder_detail(web::Path(contest): web::Path<String>) -> impl Responder {
    let mut contest_dir: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    contest_dir.pop();
    contest_dir.push("atcoder");
    contest_dir.push(contest);
    contest_dir.push("src");
    contest_dir.push("bin");

    // TODO: async map

    let entries: Vec<AtCoderEntry> = read_dir(&contest_dir)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let name = entry.file_name().to_string_lossy().to_string();

            use std::fs::File;
            use std::io::BufRead;
            use std::io::BufReader;

            let f = BufReader::new(File::open(entry.path()).unwrap());

            let mut solved = true;
            let mut keywords = Vec::new();

            const KEYWORD: &str = "// keywords :";
            for line in f.lines() {
                let line = line.unwrap();
                if line == "fn main(n: u32, _a: [u32; n]) {" {
                    solved = false;
                    break;
                }
                if line.starts_with(KEYWORD) {
                    let line = &line[KEYWORD.len()..];
                    keywords = line
                        .split(',')
                        .map(|s| s.trim())
                        .map(String::from)
                        .collect();
                }
            }

            AtCoderEntry {
                name: name.chars().nth(0).unwrap(),
                solved: solved,
                keywords: keywords,
            }
        })
        .collect();

    HttpResponse::Ok().json(entries)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(atcoder).service(atcoder_detail))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
