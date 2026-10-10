#[cfg(target_arch = "wasm32")]
mod backend {
    unsafe extern "C" {
        fn app_session_token_len() -> u32;
        fn app_session_token_take(buf_ptr: *mut u8) -> u32;
    }

    pub fn player_token() -> String {
        let len = unsafe { app_session_token_len() };
        let mut buf = vec![0u8; len as usize];
        let actual = unsafe { app_session_token_take(buf.as_mut_ptr()) };
        buf.truncate(actual as usize);
        String::from_utf8(buf).unwrap_or_default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod backend {
    /// A desktop window never reloads, so the token only has to be unique per run.
    pub fn player_token() -> String {
        format!("native-{}", (macroquad::miniquad::date::now() * 1000.0) as u64)
    }
}

pub use backend::player_token;
