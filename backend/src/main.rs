use actix_files::NamedFile;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::{get, HttpRequest, Result};
use std::path::Path;
use std::path::PathBuf;

#[get("/app/{filename:.*}")]
async fn app(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse()?;
    Ok(NamedFile::open(Path::new("dist").join(
        if path != PathBuf::from("") {
            path
        } else {
            PathBuf::from("index.html")
        },
    ))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(app))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
