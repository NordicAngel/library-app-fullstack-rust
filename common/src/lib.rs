use serde::*;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author_id: i64,
    pub image: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Author {
    #[sqlx(rename = "AuthorID")]
    pub id: i64,
    #[sqlx(rename = "Name")]
    pub name: String,
}
