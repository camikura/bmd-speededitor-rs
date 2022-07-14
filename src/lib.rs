mod speed_editor;

pub use speed_editor::{handler, key::Key, key_led::KeyLed, SpeedEditor, SpeedEditorError};

pub fn new() -> Result<SpeedEditor, SpeedEditorError> {
    let se = SpeedEditor {
        device: None,
        last_authenticated_at: None,
        current_keys: Vec::default(),
        current_key_leds: Vec::default(),
        connected_handler: |_| Ok(()),
        disconnected_handler: || Ok(()),
        keys_handler: |_, _| Ok(()),
        key_down_handler: |_, _| Ok(()),
        key_up_handler: |_, _| Ok(()),
        jog_handler: |_, _, _| Ok(()),
        unknown_handler: |_, _| Ok(()),
    };

    Ok(se)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
