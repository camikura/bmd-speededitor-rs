use std::{cell::RefCell, rc::Rc};

mod speed_editor;

pub use speed_editor::{
    handler::{ConnectedHandler, Handler},
    key::Key,
    key_led::KeyLed,
    SpeedEditor, SpeedEditorError,
};

pub fn new() -> Result<SpeedEditor, SpeedEditorError> {
    let se = SpeedEditor {
        device: None,
        last_authenticated_at: None,
        current_keys: Vec::default(),
        current_key_leds: Vec::default(),
        connected_handler: Rc::new(RefCell::new(Handler::new())),
        disconnected_handler: Rc::new(RefCell::new(Handler::new())),
        keys_handler: Rc::new(RefCell::new(Handler::new())),
        key_down_handler: Rc::new(RefCell::new(Handler::new())),
        key_up_handler: Rc::new(RefCell::new(Handler::new())),
        jog_handler: Rc::new(RefCell::new(Handler::new())),
        unknown_handler: Rc::new(RefCell::new(Handler::new())),
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
