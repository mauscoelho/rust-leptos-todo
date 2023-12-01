use crate::components::{page_wrapper::PageWrapper, todo_item::*};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let todo_items: Vec<TodoItem> = vec![
        TodoItem {
            id: 0,
            task: String::from("Take out the trash"),
            status: false,
        },
        TodoItem {
            id: 1,
            task: String::from("Make the bed"),
            status: false,
        },
        TodoItem {
            id: 2,
            task: String::from("Fold the laundry"),
            status: false,
        },
        TodoItem {
            id: 3,
            task: String::from("Do the dishes"),
            status: false,
        },
    ];

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/example.css" />
        <Title text="Cargo Leptos" />
        <main class="my-0 mx-auto max-w-3xl text-center">
            <PageWrapper>
                <div id="todo_items">
                    <For
                    each=move || { todo_items.clone() }
                    key=|task| task.id
                    view=move |cx, task: TodoItem| {
                    view! {
                        cx,
                        <TodoItem todo_item={task} />
                    }
                    }
                    />
                </div>
            </PageWrapper>
        </main>
    }
}

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "hydrate")]
#[wasm_bindgen(module = "/js/foo.js")]
extern "C" {
    pub fn message() -> String;
}

#[cfg(not(feature = "hydrate"))]
#[allow(dead_code)]
pub fn message() -> String {
    "Rust".to_string()
}
