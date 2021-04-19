mod generated;
extern crate wapc_guest as guest;
pub use generated::*;
use guest::prelude::*;

#[no_mangle]
pub fn wapc_init() {
    console_log("WASM: WAPC init");
    Handlers::register_callback(callback);
}

fn callback(
    binding: String,
    namespace: String,
    operation: String,
    msg: String,
) -> HandlerResult<String> {
    console_log(&format!(
        "WASM: received ({},{},{},{})",
        binding, namespace, operation, msg
    ));
    let result = host_call(&binding, &namespace, &operation, &serialize(&msg)?)?;
    console_log(&format!("WASM: Got result from host call {:?}", result));
    let new_string: String = deserialize(&result)?;
    Ok(new_string)
}
