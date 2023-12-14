#![feature(result_option_inspect)]
use yew::prelude::*;

use crate::search_book::SearchBook;
mod add_book_popup;
mod search_book;

#[function_component]
fn App() -> Html {
    let show_add_book = use_state(|| false);
    let onclick = {
        let show_add_book = show_add_book.clone();
        Callback::from(move |_| {
            let show_add_book = show_add_book.clone();
            show_add_book.set(true);
        })
    };

    let on_finished_form = {
        let show_add_book = show_add_book.clone();
        Callback::from(move |bool| {
            show_add_book.set(bool);
        })
    };
    html! {
        <>
            <ul>
        <li>
        <a {onclick}>{"Add Book"}</a>
        </li>
            </ul>

            <div class="inner">
            <SearchBook/>
            if *show_add_book {
                <add_book_popup::Popup {on_finished_form}/>
            }
        // <FeedbackForm/>
            </div>
        </>
    }
}

// #[function_component]
// fn Popup() -> Html {
//     // let title = use_state(String::new());
//     let title_ref = use_node_ref();

//     let onsubmit = {
//         // let title = title.clone();
//         let title_ref = title_ref.clone();

//         Callback::from(move |event: SubmitEvent| {
//             event.prevent_default();
//             let x = event.target().unwrap().value_of().to_string();
//             let target = event.target().unwrap();
//             let value = wasm_bindgen_futures::wasm_bindgen::JsCast::unchecked_into::<
//                 web_sys::HtmlInputElement,
//             >(target)
//             .value();
//             log_1(&value.into());
//         })
//     };
//     let oninput = Callback::from(|event: InputEvent| {
//         let data = event.data().unwrap_or_default();
//         log_1(&data.into());
//     });
//     html! {
//         <form class="popup" {onsubmit}>
//             {"popup"}
//             <input
//         type="text"
//         ref={title_ref}
//         placeholder="Title"
//         {oninput}
//         />
//         <button
//             type="submit">
//             {"Create"}
//         </button>
//         </form>
//     }
// }

#[function_component]
pub fn FeedbackForm() -> Html {
    let text = use_state(String::new);
    let rating = use_state(|| 10_u8);
    let min = use_state(|| 10);
    let message = use_state(|| Option::<String>::None);

    let text_input_ref = use_node_ref();

    let handle_input = {
        let text = text.clone();
        let message = message.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = wasm_bindgen_futures::wasm_bindgen::JsCast::unchecked_into::<
                web_sys::HtmlInputElement,
            >(target)
            .value();
            message.set(None);
            text.set(value);
        })
    };

    let on_submit = {
        let rating = rating.clone();
        let text = text.clone();
        let message = message.clone();
        let text_input_ref = text_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            if text.trim().len() < *min {
                message.set(Some("Text must be at least 10 characters".to_string()));
                return;
            }

            let val: wasm_bindgen_futures::wasm_bindgen::JsValue = (*text.to_string()).into();
            web_sys::console::log_1(&val);

            let text_input = text_input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            text_input.set_value("");
            text.set(String::new());
            rating.set(10);
        })
    };

    html! {
        <div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
            <header class="max-w-md mx-auto">
                <h2 class="text-center text-2xl font-bold">{"How would you rate your service with us?"}</h2>
            </header>
            <form onsubmit={on_submit}>
                                <div class="flex border rounded-lg my-4 px-2 py-3">
                    <input
                        type="text"
                        ref={text_input_ref}
                        oninput={handle_input}
                        class="flex-grow border-none text-lg focus:outline-none"
                        placeholder="Tell us something that keeps you coming back"
                    />
                <button
                    type="submit">
                    {"Send"}
                </button>
                </div>
                {if let Some(msg) = message.as_ref() {
                    html! { <div class="pt-3 text-center text-purple-600">{msg.clone()}</div> }
                } else {
                    html! {}
                }}
            </form>
        </div>
    }
}

fn main() { yew::Renderer::<App>::new().render(); }
