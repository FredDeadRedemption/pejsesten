use std::cell::RefCell;

thread_local! {
    static READ_RESULT: RefCell<Option<Result<String, String>>> = const { RefCell::new(None) };
}

pub fn write(text: &str) {
    miniquad::window::clipboard_set(text);
}

pub fn request_read() {
    let result = miniquad::window::clipboard_get()
        .map(Ok)
        .unwrap_or(Err("clipboard empty".to_string()));
    READ_RESULT.with(|cell| *cell.borrow_mut() = Some(result));
}

pub fn take_read_result() -> Option<Result<String, String>> {
    READ_RESULT.with(|cell| cell.borrow_mut().take())
}
