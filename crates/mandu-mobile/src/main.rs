use dioxus::prelude::*;
use mandu_mobile::App;

fn app_root() -> Element {
    rsx! {
        App { initial_vault: None }
    }
}

fn main() {
    dioxus::launch(app_root);
}
