use common::{Author, Book};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, use_node_ref, use_state, Callback, Html, Properties};

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
        Callback::from(move |event: InputEvent| {
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
    props
        .books
        .iter()
        .map(|book| html! {<BookToHtml book={book.clone()}/>})
        .collect()
}

#[derive(Properties, PartialEq)]
struct HtmlBookProp {
    book: Book,
}

#[function_component]
fn BookToHtml(props: &HtmlBookProp) -> Html {
    let author_name = use_state(|| "...Loading...".to_owned());
    let author_id = props.book.author_id;
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
            <p class="book-title"> {props.book.title.to_owned()}</p>
        <p class="by-line">{format!("By {}", (*author_name))}</p>
        </div>
    }
}
