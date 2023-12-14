use common::{Author, Book};
use gloo_net::http::Request;
use input_yew::CustomInput;
use regex::Regex;
use serde_json;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::{function_component, html, use_node_ref, use_state, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_finished_form: Callback<bool>,
}
async fn add_book(book: &Book) {
    Request::post("http://127.0.0.1:8080/api/book")
        .body(serde_json::to_string(book).unwrap())
        .unwrap()
        .send()
        .await
        .unwrap();
}

async fn add_book_and_author(book: &Book, author_name: &str) {
    Request::post("http://127.0.0.1:8080/api/author")
        .body(author_name.to_owned())
        .unwrap()
        .send()
        .await
        .unwrap();
    let author = serde_json::from_str::<Author>(
        &Request::get(&format!(
            "http://127.0.0.1:8080/api/author/name/{}",
            author_name
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap(),
    )
    .unwrap();
    let book = Book {
        isbn: book.isbn.to_owned(),
        title: book.title.to_owned(),
        author_id: author.id,
        image: book.image.to_owned(),
        description: book.description.to_owned(),
    };
    Request::post("127.0.0.1:8080/api/book")
        .body(serde_json::to_string(&book).unwrap())
        .unwrap()
        .send()
        .await
        .unwrap();
}

#[function_component]
pub(crate) fn Popup(props: &Props) -> Html {
    let on_finished_form = props.on_finished_form.clone();
    let isbn_ref = use_node_ref();
    let isbn_handle = use_state(String::default);
    let val_isbn_handle = use_state(|| true);

    let title_ref = use_node_ref();
    let title_handle = use_state(String::default);
    let val_title_handle = use_state(|| true);

    let list_ref = use_node_ref();
    let authors_vec: yew::UseStateHandle<Option<Vec<Author>>> = use_state(|| None);
    let authors_html = use_state(|| html! {});

    let onsubmit = {
        let list_ref = list_ref.clone();
        let isbn_handle = isbn_handle.clone();
        let title_handle = title_handle.clone();
        let authors_vec = authors_vec.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let isbn_handle = isbn_handle.clone();
            let title_handle = title_handle.clone();
            if let Some(input) = list_ref.cast::<HtmlInputElement>() {
                let author_string = input.value();
                let authors_vec = (*authors_vec).clone().unwrap_or(Vec::new());
                let picked_author = authors_vec
                    .iter()
                    .filter(|author| author.name == author_string)
                    .nth(0);
                match picked_author {
                    Some(author) => {
                        let author = author.clone();
                        spawn_local(async move {
                            let author = author.clone();
                            let book = Book {
                                isbn: isbn_handle.to_string(),
                                title: title_handle.to_string(),
                                author_id: author.id,
                                image: None,
                                description: None,
                            };
                            add_book(&book).await
                        });
                        on_finished_form.emit(false);
                    },
                    None => {
                        spawn_local(async move {
                            let book = Book {
                                isbn: isbn_handle.to_string(),
                                title: title_handle.to_string(),
                                author_id: 0,
                                image: None,
                                description: None,
                            };
                            add_book_and_author(&book, &author_string).await
                        });
                        on_finished_form.emit(false);
                    },
                }
            }
        })
    };

    {
        let authors_html = authors_html.clone();
        spawn_local(async move {
            let request = Request::get("http://127.0.0.1:8080/api/author")
                .send()
                .await
                .expect("fetch fail")
                .text()
                .await
                .unwrap_or_default();
            authors_vec.set(Some(serde_json::from_str::<Vec<Author>>(&request).unwrap()));
            authors_html.set(
                authors_vec
                    .as_ref()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|author| {
                        let author = author;
                        html! {
                            <option key={author.id} value={author.name.to_string()}/>
                        }
                    })
                    .collect::<Html>(),
            );
        });
    }
    html! {
        <form class="popup" onsubmit={onsubmit}>
            <CustomInput
                input_type="text"
                label="ISBN"
                input_ref={isbn_ref}
                input_handle={isbn_handle}
                input_valid_handle={val_isbn_handle}
                validate_function={|i: String |Regex::new(r"\d{10,13}").unwrap().is_match(&i)}
            />
            <CustomInput
                input_type="text"
                label="Title"
                input_ref={title_ref}
                input_handle={title_handle}
                input_valid_handle={val_title_handle}
                validate_function={|_|true}
            />
        <datalist id="authors">
        {(*authors_html).clone()}
        </datalist>
        <lable>{"Author"}</lable>
        <div>
        <input list="authors"
        ref={list_ref}
        />
        </div>
        <button type="submit">{"Add Book"}</button>
        </form>
    }
}
