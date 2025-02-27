#[path = "js_container.rs"] mod js_container;
use std::borrow::Cow;

fn main() {
    fn load_test() -> Cow<'static, str> {
        let bytes = include_bytes!("test.js");
        let js = String::from_utf8_lossy(bytes);
        js
    }
    let js = load_test();
    js_container::run_js(js);
}