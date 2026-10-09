//! One log channel for both targets: wasm has no stdout, so it goes through a JS plugin.

#[cfg(not(target_arch = "wasm32"))]
pub fn line(message: &str) {
    eprintln!("{message}");
}

#[cfg(target_arch = "wasm32")]
pub fn line(message: &str) {
    unsafe extern "C" {
        fn app_log(ptr: *const u8, len: u32);
    }
    let bytes = message.as_bytes();
    unsafe { app_log(bytes.as_ptr(), bytes.len() as u32) };
}

macro_rules! log {
    ($($arg:tt)*) => { crate::log::line(&format!($($arg)*)) };
}
