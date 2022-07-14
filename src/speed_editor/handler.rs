use super::{Key, SpeedEditor, SpeedEditorError};

pub type ConnectedHandler = fn(se: &SpeedEditor) -> Result<(), SpeedEditorError>;
pub type DisconnectedHandler = fn() -> Result<(), SpeedEditorError>;
pub type KeysHandler = fn(se: &SpeedEditor, keys: Vec<Key>) -> Result<(), SpeedEditorError>;
pub type KeyDownHandler = fn(se: &SpeedEditor, key: Key) -> Result<(), SpeedEditorError>;
pub type KeyUpHandler = fn(se: &SpeedEditor, key: Key) -> Result<(), SpeedEditorError>;
pub type JogHandler = fn(se: &SpeedEditor, mode: u8, value: i32) -> Result<(), SpeedEditorError>;
pub type UnknownHandler = fn(se: &SpeedEditor, data: &[u8]) -> Result<(), SpeedEditorError>;
