use actix_files::NamedFile;
use actix_web::{get, App, HttpRequest, HttpServer, Result};
use crud_api::*;
use std::path::{Path, PathBuf};
mod crud_api;

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
    HttpServer::new(|| {
        App::new()
            .service(app)
            .service(all_authors)
            .service(add_author)
            .service(add_book)
            .service(author_by_name)
            .service(search_book)
            .service(author_by_id)
            .service(delete_book)
            .service(update_book)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
