use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author_id: i64,
    pub image: Option<Vec<u8>>,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct Author {
    #[sqlx(rename = "AuthorID")]
    pub id: i64,
    #[sqlx(rename = "Name")]
    pub name: String,
}
