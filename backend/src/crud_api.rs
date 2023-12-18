use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use common::{Author, Book};
use sqlx::{Error, MySql, MySqlPool, Pool};

pub(crate) async fn connect() -> Result<Pool<MySql>, Error> {
    MySqlPool::connect("mysql://server:A Very Secure Password he he@localhost:3306/library").await
}

macro_rules! catch_result {
    ($e:block) => {
        async {
            $e;
            anyhow::Ok(())
        }
        .await
    };
    ($e:expr) => {
        async { anyhow::Ok($e) }.await
    };
}

#[post("/api/book")]
pub(crate) async fn add_book(req_body: String) -> HttpResponse {
    match catch_result!({
        let book: Book = serde_json::from_str::<Book>(&req_body)?;
        sqlx::query!(
            "INSERT INTO Books (ISBN, Title, AuthorID, Image, Description) VALUES (?, ?, ?, ?, ?)",
            book.isbn,
            book.title,
            book.author_id,
            book.image,
            book.description
        )
        .execute(&connect().await?)
        .await?;
    }) {
        Ok(_) => HttpResponse::Created().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/api/author")]
pub(crate) async fn add_author(req_body: String) -> HttpResponse {
    match catch_result!(
        sqlx::query!("INSERT INTO Authors (Name) VALUES (?)", req_body)
            .execute(&connect().await?)
            .await?
    ) {
        Ok(_) => HttpResponse::Created().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[put("/api/book/{isbn}")]
pub(crate) async fn update_book(info: web::Path<String>, req_body: String) -> HttpResponse {
    match catch_result!({
        let book: Book = serde_json::from_str::<Book>(&req_body)?;
        sqlx::query!(
            "UPDATE Books SET ISBN = ?, Title = ?, AuthorID = ?, Image = ?, Description = ? WHERE ISBN = ?",
            book.isbn,
            book.title,
            book.author_id,
            book.image,
            book.description,
            info.into_inner()
        )
        .execute(&connect().await?)
        .await?;
    }) {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[delete("/api/book/{isbn}")]
pub(crate) async fn delete_book(info: web::Path<String>) -> HttpResponse {
    match catch_result!(
        sqlx::query!("DELETE FROM Books WHERE ISBN = ?", info.into_inner())
            .execute(&connect().await?)
            .await?
    ) {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

macro_rules! api_read_all {
    ($type:ty,$query:expr,$($info:expr),*) => {{
        let err_msg: HttpResponse = HttpResponse::InternalServerError().into();
        let conn = connect().await;
        if let Err(_) = conn {
            return err_msg;
        }
        let query = sqlx::query_as!($type, $query, $($info),*)
            .fetch_all(&conn.unwrap())
            .await;
        if let Err(_) = query {
            return err_msg;
        }
        match serde_json::to_string(&query.unwrap()) {
            Ok(json) => HttpResponse::Ok().body(json),
            Err(_) => HttpResponse::InternalServerError().into(),
        }
    }};
}
macro_rules! api_read_first {
    ($type:ty,$query:expr,$($info:expr),*) => {{
        let err_msg: HttpResponse = HttpResponse::InternalServerError().into();
        let conn = connect().await;
        if let Err(_) = conn {
            return err_msg;
        }
        let query = sqlx::query_as!($type, $query, $($info),*)
            .fetch_one(&conn.unwrap())
            .await;
        if let Err(_) = query {
            return err_msg;
        }
        match serde_json::to_string(&query.unwrap()) {
            Ok(json) => HttpResponse::Ok().body(json),
            Err(_) => HttpResponse::InternalServerError().into(),
        }
    }};
}

#[get("/api/author")]
async fn all_authors() -> HttpResponse {
    api_read_all!(Author, "SELECT Name as name, AuthorId as id FROM Authors;",)
}

#[get("api/author/name/{name}")]
async fn author_by_name(info: web::Path<String>) -> HttpResponse {
    api_read_all!(
        Author,
        "SELECT Name as name, AuthorID as id FROM Authors WHERE NAME = ?",
        info.into_inner()
    )
}

#[get("api/author/id/{id}")]
async fn author_by_id(info: web::Path<String>) -> HttpResponse {
    api_read_first!(
        Author,
        "SELECT Name as name, AuthorID as id FROM Authors WHERE AuthorID = ?",
        info.into_inner()
    )
}

#[get("/api/book/search/{term}")]
async fn search_book(info: web::Path<String>) -> HttpResponse {
    api_read_all!(
        Book,
        "SELECT ISBN as isbn, \
         Title as title, \
         AuthorId as author_id, \
        Image as image, \
        Description as description \
        FROM Books \
        WHERE isbn = ? OR \
        title like ? OR \
        AuthorId IN \
        (SELECT AuthorId FROM Authors WHERE Name LIKE ?)",
        format!("{}", info),
        format!("%{}%", info),
        format!("%{}%", info)
    )
}
