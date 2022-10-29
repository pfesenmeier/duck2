use gloo::worker::{HandlerId, Worker, WorkerScope};

use pulldown_cmark::{html, Parser};

#[derive(Debug)]
pub enum Msg<T> {
    Respond { output: T, id: HandlerId },
}

pub struct MarkdownWorker {}

impl Worker for MarkdownWorker {
    // The Markdown Markup to Render.
    type Input = String;

    type Message = Msg<String>;

    // The Rendered Html Output.
    type Output = String;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        let Msg::Respond { output, id } = msg;

        scope.respond(id, output);
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, who: HandlerId) {
        let parser = Parser::new(&msg);

        let mut output = String::new();
        html::push_html(&mut output, parser);

        scope.send_message(Msg::Respond { output, id: who });
    }
}
