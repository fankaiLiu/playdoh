use deno_core::{anyhow::Error, op};
use serde::Deserialize;

use crate::jsruntime::JS_DATA;

#[derive(Debug, Deserialize)]
pub struct MsgArgs {
    id: String,
    key: String,
    msg: String,
}

#[op]
fn op_add_msg(arg: MsgArgs) -> Result<(), Error> {
    JS_DATA
        .lock()
        .unwrap()
        .get_mut(&arg.id)
        .unwrap()
        .as_object_mut()
        .unwrap()
        .insert(arg.key, arg.msg.clone().into());
    Ok(())
}
