use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

use gloo::utils::document;
use gloo::worker::Spawnable;

use js_sys::Promise;
use wasm_bindgen_futures::{spawn_local, JsFuture};

use gloo::worker::Registrable;

pub mod markdown_worker;

use markdown_worker::MarkdownWorker;

static MARKDOWN_CONTENT: &str = r#"
## Hello
This content is *rendered* by a **web worker**.
"#;

fn spawn_worker() {
    let root = document()
        .query_selector("#root")
        .ok()
        .flatten()
        .expect_throw("failed to query root element");

    let bridge = MarkdownWorker::spawner()
        .callback(move |m| {
            root.set_inner_html(&m);
        })
        .spawn("/example_markdown_worker.js");

    bridge.send(MARKDOWN_CONTENT.to_owned());

    spawn_local(async move {
        bridge.send(MARKDOWN_CONTENT.to_owned());

        // We need to hold the bridge until the worker resolves.
        let promise = Promise::new(&mut |_, _| {});
        let _ = JsFuture::from(promise).await;
    });

    gloo::console::log!("spawning worker...");
    // MarkdownWorker::registrar().register();
}

pub fn app(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div {
            id: "root"

        }
        button {
            onclick: |_| spawn_worker(),
            "Spawn Worker"
        }
    })
}
