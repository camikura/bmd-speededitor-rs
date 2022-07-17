mod speed_editor;

pub use speed_editor::{
    handler::{ConnectedHandler, Handler},
    key::Key,
    key_led::KeyLed,
    SpeedEditor, SpeedEditorError,
};

pub fn new() -> Result<SpeedEditor, SpeedEditorError> {
    Ok(SpeedEditor {
        device: None,
        last_authenticated_at: None,
        current_keys: Vec::default(),
        current_key_leds: Vec::default(),
        connected_handler: Handler::new(),
        disconnected_handler: Handler::new(),
        keys_handler: Handler::new(),
        key_handler: Handler::new(),
        key_down_handler: Handler::new(),
        key_up_handler: Handler::new(),
        jog_handler: Handler::new(),
        unknown_handler: Handler::new(),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
