use leptos::*;

#[derive(Clone)]
pub struct TodoItem {
    pub id: u32,
    pub task: String,
    pub status: bool,
}

/// A todo item component
#[component]
pub fn TodoItem(cx: Scope, todo_item: TodoItem) -> impl IntoView {
    view! {
    cx,
    <div class="flex justify-between items-center">
        <span>{todo_item.task}</span>
        <div class="flex justify-between w-fit sm:w-1/3">
            <button class="hover:cusor-pointer">"Complete"</button>
            <button class="hover:cusor-pointer ml-4 sm:ml-0">"Delete"</button>
        </div>
    </div>
    }
}
