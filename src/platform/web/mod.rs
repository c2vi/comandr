use web_sys::console;
use web_sys::js_sys::wasm_bindgen;
use web_sys::EventTarget;
use web_sys::js_sys::Function;
use wasm_bindgen::prelude::*;

use crate::Comandr;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn init(comandr: &mut Comandr) {
    console_log!("hello from comandr");
    console::log_1(&"hi from comandr not macro".into());

    let js_init_fn = r#"
        window.addEventListener('keypress', function(e){
            if (e.ctrlKey && e.keyCode == 13) {
                window.comandr.fns.main_show()
            }
        }, false);

        window.comandr = {}
        window.comandr.fns = {}
    "#;
    let js_init_jsfn = Function::new_no_args(js_init_fn);

    js_init_jsfn.call0(&web_sys::wasm_bindgen::JsValue::NULL);
    

    //EventTarget::new().expect("failed to create a EventTarget").add_event_listener_with_callback("keypress", &js_init_jsfn);
}

#[wasm_bindgen]
pub fn main_show() {
    console::log_1(&"main_show".into());
}


