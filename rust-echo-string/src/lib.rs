mod generated;
extern crate wapc_guest as guest;
pub use generated::*;
use guest::prelude::*;

#[no_mangle]
pub fn wapc_init() {
    console_log("WASM: WAPC init");
    Handlers::register_echo(echo);
}

fn echo(msg: String) -> HandlerResult<String> {
    console_log(&format!("WASM: Got string {}", msg));
    Ok(msg)
}
