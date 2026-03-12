mod backend;
mod pages;
mod shared;

use dioxus::prelude::*;
use dioxus_router::{Router};

fn main() {
    launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        Router::<pages::router::Route> {}
    }
}
