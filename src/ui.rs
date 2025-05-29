use ribir::prelude::*;

pub fn run() {
    App::run(fn_widget! { @Text { text: "Hello world!" }});
}