mod speed_editor;

use speed_editor::{SpeedEditor, SpeedEditorError};

pub fn new() -> Result<SpeedEditor, SpeedEditorError> {
    let se = SpeedEditor {
        device: None,
        last_authenticated_at: None,
        current_keys: Vec::default(),
        connected_callback: || Ok(()),
        disconnected_callback: || Ok(()),
        keys_callback: |_| Ok(()),
        key_down_callback: |_| Ok(()),
        key_up_callback: |_| Ok(()),
        jog_callback: |_| Ok(()),
        unknown_callback: |_| Ok(()),
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
