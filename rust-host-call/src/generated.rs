extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
pub struct Host {
    binding: String,
}

#[cfg(feature = "guest")]
impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    pub fn callback(
        &self,
        binding: String,
        namespace: String,
        operation: String,
        msg: String,
    ) -> HandlerResult<String> {
        let input_args = CallbackArgs {
            binding,
            namespace,
            operation,
            msg,
        };
        host_call(
            &self.binding,
            "greeting",
            "callback",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<String>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    pub fn register_callback(f: fn(String, String, String, String) -> HandlerResult<String>) {
        *CALLBACK.write().unwrap() = Some(f);
        register_function(&"callback", callback_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static::lazy_static! {
static ref CALLBACK: std::sync::RwLock<Option<fn(String, String, String, String) -> HandlerResult<String>>> = std::sync::RwLock::new(None);
}

#[cfg(feature = "guest")]
fn callback_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<CallbackArgs>(input_payload)?;
    let lock = CALLBACK.read().unwrap().unwrap();
    let result = lock(input.binding, input.namespace, input.operation, input.msg)?;
    serialize(result)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct CallbackArgs {
    #[serde(rename = "binding")]
    pub binding: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "msg")]
    pub msg: String,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
