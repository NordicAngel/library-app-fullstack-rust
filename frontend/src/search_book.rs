use common::{Author, Book};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, use_node_ref, use_state, Callback, Html, Properties};

use crate::search_book::edit_book_popup::EditBookPopup;

mod edit_book_popup;

#[derive(Properties, PartialEq)]
struct HtmlBooksProps {
    books: Vec<Book>,
}
#[function_component]
pub(crate) fn SearchBook() -> Html {
    let books_vec = use_state(|| Vec::<Book>::new());
    let search_handle = use_state(|| String::new());
    let search_ref = use_node_ref();
    let oninput = {
        let books_vec = books_vec.clone();
        let search_handle = search_handle.clone();
        let search_ref = search_ref.clone();
        Callback::from(move |_event: InputEvent| {
            let books_vec = books_vec.clone();
            let search_handle = search_handle.clone();
            if let Some(input) = search_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                search_handle.set((&value).to_owned());
                spawn_local(async move {
                    if value.len() > 2 {
                        let request = Request::get(&format!(
                            "http://127.0.0.1:8080/api/book/search/{}",
                            value
                        ))
                        .send()
                        .await
                        .expect("fetch fail")
                        .text()
                        .await
                        .unwrap();
                        books_vec.set(
                            serde_json::from_str::<Vec<Book>>(&request)
                                .expect(&format!("req: {}", request)),
                        );
                    } else {
                        books_vec.set(Vec::new());
                    }
                });
            }
        })
    };
    html! {
        <div class="book-search">
        <input type="text" class="search-box" {oninput} ref={search_ref} placeholder="Search by title, author or ISBN"/>
            <div class="book-list">
                <BooksToHtml books={(*books_vec).clone()}/>
            </div>
        </div>
    }
}

#[function_component]
fn BooksToHtml(props: &HtmlBooksProps) -> Html {
    let book_to_edit = use_state(|| Option::<Book>::None);
    let edit = {
        let book_to_edit = book_to_edit.clone();
        Callback::from(move |book: Book| {
            let book_to_edit = book_to_edit.clone();
            book_to_edit.set(Some(book));
        })
    };
    let close_edit = {
        let book_to_edit = book_to_edit.clone();
        Callback::from(move |_| {
            let book_to_edit = book_to_edit.clone();
            book_to_edit.set(None);
        })
    };
    let books = props
        .books
        .iter()
        .map(|book| html! {<BookToHtml book={book.clone()} edit_callback={edit.clone()}/>})
        .collect::<Html>();
    html! {
        <>
        {books}
        if (*book_to_edit).is_some()
        {<EditBookPopup book={(*book_to_edit).clone().unwrap()} close_form={close_edit}/>}
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct HtmlBookProp {
    book: Book,
    edit_callback: Callback<Book>,
}

#[function_component]
fn BookToHtml(props: &HtmlBookProp) -> Html {
    let author_name = use_state(|| "...Loading...".to_owned());
    let author_id = props.book.author_id;
    let props = props.clone();
    let onclick = {
        let props = props.clone();
        move |_| {
            let props = props.clone();
            props.edit_callback.emit(props.book.clone())
        }
    };
    {
        let author_name = author_name.clone();
        spawn_local(async move {
            let author_name = author_name.clone();
            let request = Request::get(&format!(
                "http://127.0.0.1:8080/api/author/id/{}",
                author_id
            ))
            .send()
            .await
            .expect("fetch fail")
            .text()
            .await
            .unwrap();
            let author =
                serde_json::from_str::<Author>(&request).expect(&format!("req: {}", request));
            author_name.set(author.name);
        });
    }
    html! {
        <div class="book">
        <div>
            <p class="book-title"> {props.book.title}</p>
        <p class="by-line">{format!("By {}", (*author_name))}</p>
        </div>
        <button {onclick}>{"Edit Book"}</button>
        </div>
    }
}
