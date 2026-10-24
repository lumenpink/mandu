use dioxus::prelude::*;
use mandu_mobile::App;

#[component]
pub fn WebApp() -> Element {
    rsx! {
        App { initial_vault: None }
    }
}

pub fn run_web() {
    dioxus::launch(WebApp);
}
