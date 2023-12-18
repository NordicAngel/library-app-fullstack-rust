use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use gloo_net::http::Request;
use input_yew::CustomInput;
use regex::Regex;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{js_sys, DragEvent, HtmlInputElement, SubmitEvent};
use yew::{html, use_node_ref, use_state, Callback, Html};

use yew::{function_component, Properties};

use common::{Author, Book};

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct EditBookProps {
    pub(crate) book: Book,
    pub(crate) close_form: Callback<()>,
}

#[function_component]
pub(crate) fn EditBookPopup(props: &EditBookProps) -> Html {
    let close_form = props.close_form.clone();
    let isbn_ref = use_node_ref();
    let isbn_handle = use_state(|| props.book.isbn.clone());
    let val_isbn_handle = use_state(|| true);

    let title_ref = use_node_ref();
    let title_handle = use_state(|| props.book.title.clone());
    let val_title_handle = use_state(|| true);

    let desc_ref = use_node_ref();
    let desc_handle = use_state(|| props.book.description.clone().unwrap_or_default());
    let val_desc_handle = use_state(|| true);

    let list_ref = use_node_ref();
    let list_start_val = use_state(|| "Loading".to_owned());
    let authors_vec: yew::UseStateHandle<Option<Vec<Author>>> = use_state(|| None);
    let authors_html = use_state(|| html! {});

    let image_file = use_state(|| props.clone().book.image);

    let onclick_cancel = {
        let close_form = props.close_form.clone();
        move |_| close_form.emit(())
    };

    let upload = {
        let image_file = image_file.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            let image_file = image_file.clone();
            spawn_local(async move {
                let event = event.clone();
                let file = event
                    .data_transfer()
                    .unwrap()
                    .files()
                    .unwrap()
                    .item(0)
                    .unwrap();
                let future: JsFuture = file.array_buffer().into();
                let vec8 = js_sys::Uint8Array::new(&future.await.unwrap()).to_vec();
                image_file.set(Some(vec8));
            })
        })
    };

    let onsubmit = {
        let prev_isbn = props.book.isbn.clone();
        let list_ref = list_ref.clone();
        let isbn_handle = isbn_handle.clone();
        let title_handle = title_handle.clone();
        let authors_vec = authors_vec.clone();
        let image_file = image_file.clone();
        let desc_handle = desc_handle.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let image_file = image_file.clone();
            let prev_isbn = prev_isbn.clone();
            let isbn_handle = isbn_handle.clone();
            let title_handle = title_handle.clone();
            let desc_handle = desc_handle.clone();
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
                                image: (*image_file).clone(),
                                description: if desc_handle.trim().len() > 0 {
                                    Some((*desc_handle).clone())
                                } else {
                                    None
                                },
                            };
                            update_book(&prev_isbn, &book).await
                        });
                        close_form.emit(());
                    },
                    None => {
                        spawn_local(async move {
                            let book = Book {
                                isbn: isbn_handle.to_string(),
                                title: title_handle.to_string(),
                                author_id: 0,
                                image: (*image_file).clone(),
                                description: if desc_handle.trim().len() > 0 {
                                    Some((*desc_handle).clone())
                                } else {
                                    None
                                },
                            };
                            add_author_update_book(&prev_isbn, &book, &author_string).await
                        });
                        close_form.emit(());
                    },
                }
            }
        })
    };

    let onclick_delete = {
        let book = props.book.clone();
        move |_| {
            let book = book.clone();
            spawn_local(async move {
                let book = book.clone();
                delete_book(&book.isbn).await
            });
        }
    };

    {
        let props = props.clone();
        let list_start_val = list_start_val.clone();
        let authors_html = authors_html.clone();
        spawn_local(async move {
            let props = props.clone();
            let list_start_val = list_start_val.clone();
            let request = Request::get("http://127.0.0.1:8080/api/author")
                .send()
                .await
                .expect("fetch fail")
                .text()
                .await
                .unwrap_or_default();
            let auth_vec = serde_json::from_str::<Vec<Author>>(&request).unwrap();
            list_start_val.set(
                get_author_by_id(&auth_vec, &props.book.author_id)
                    .unwrap()
                    .name,
            );
            authors_vec.set(Some(auth_vec));
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
            if image_file.is_some() {<img src={format!("data:png;base64,{}", STANDARD.encode((*image_file).clone().unwrap()))} />}
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
            <CustomInput
                input_type="textarea"
                label="Description"
                input_ref={desc_ref}
                input_handle={desc_handle}
                input_valid_handle={val_desc_handle}
                validate_function={|_|true}
            />
        <datalist id="authors">
        {(*authors_html).clone()}
        </datalist>
        <lable>{"Author"}</lable>
        <input list="authors"
        value={(*list_start_val).clone()}
        ref={list_ref}
        />
        <div ondrop={upload} class="image-upload">
            <p>{"Drag file here."}</p>
        </div>
        <button onclick={onclick_cancel}>{"Cancel"}</button>
        <button type="submit">{"Update Book"}</button>
        <button onclick={onclick_delete} class="delete">{"Delete"}</button>
        </form>
    }
}

async fn add_author_update_book(prev_isbn: &str, book: &Book, author_name: &str) {
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
    update_book(prev_isbn, &book).await;
}

async fn update_book(prev_isbn: &str, book: &Book) {
    Request::put(&format!("http://127.0.0.1:8080/api/book/{}", prev_isbn))
        .body(serde_json::to_string(book).unwrap())
        .unwrap()
        .send()
        .await
        .unwrap();
}

async fn delete_book(isbn: &str) {
    Request::delete(&format!("127.0.0.1/api/book/{}", isbn))
        .send()
        .await
        .unwrap();
}

fn get_author_by_id(authors: &Vec<Author>, id: &i64) -> Option<Author> {
    authors
        .iter()
        .filter(|author| author.id == *id)
        .map(|author| (*author).clone())
        .nth(0)
}
