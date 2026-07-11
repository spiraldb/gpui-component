use gpui::App;

#[cfg(not(target_family = "wasm"))]
pub(crate) fn write_clipboard_text(cx: &App, text: String) {
    use gpui::ClipboardItem;

    cx.write_to_clipboard(ClipboardItem::new_string(text));
}

#[cfg(target_family = "wasm")]
pub(crate) fn write_clipboard_text(_: &App, text: String) {
    if let Some(window) = web_sys::window() {
        // Keep this in the input event turn for the browser's user-activation check.
        let _ = window.navigator().clipboard().write_text(&text);
    }
}
