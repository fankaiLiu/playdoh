use deno_core::{include_js_files, Extension};

use self::{
    context::message::op_add_msg,
    fetch::{op_decode_utf8, op_fetch},
    notice::email::op_send_email,
};

pub mod context;
pub mod fetch;
pub mod notice;
pub fn init() -> Extension {
    Extension::builder()
        .js(include_js_files!(prefix "fetch","glue.js",))
        .ops(vec![
            op_fetch::decl(),
            op_decode_utf8::decl(),
            op_send_email::decl(),
            op_add_msg::decl(),
        ])
        .state(move |state| {
            state.put::<reqwest::Client>(reqwest::Client::new());
            Ok(())
        })
        .build()
}
