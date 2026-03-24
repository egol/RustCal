use cursive::backend::Backend;
use cursive::backends;

#[cfg(feature = "native-ui")]
pub fn backend() -> Box<dyn Backend> {
    backends::crossterm::Backend::init().unwrap()
}

#[cfg(target_family = "wasm")]
pub fn backend() -> Box<dyn Backend> {
    backends::wasm::Backend::init().unwrap()
}
