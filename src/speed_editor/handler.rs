use super::{Key, SpeedEditor, SpeedEditorResult};

pub type ConnectedCallback = Box<dyn FnMut(&SpeedEditor) -> SpeedEditorResult>;
pub type DisconnectedCallback = Box<dyn FnMut() -> SpeedEditorResult>;
pub type KeysCallback = Box<dyn FnMut(&SpeedEditor, Vec<Key>) -> SpeedEditorResult>;
pub type KeyDownCallback = Box<dyn FnMut(&SpeedEditor, Key) -> SpeedEditorResult>;
pub type KeyUpCallback = Box<dyn FnMut(&SpeedEditor, Key) -> SpeedEditorResult>;
pub type JogCallback = Box<dyn FnMut(&SpeedEditor, u8, i32) -> SpeedEditorResult>;
pub type UnknownCallback = Box<dyn FnMut(&SpeedEditor, &[u8]) -> SpeedEditorResult>;

pub trait Handler {
    fn new() -> Self;
}

pub struct ConnectedHandler {
    pub callbacks: Vec<ConnectedCallback>,
}

impl ConnectedHandler {
    pub fn call(&mut self, se: &SpeedEditor) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(se)?;
        }
        Ok(())
    }
}

impl Handler for ConnectedHandler {
    fn new() -> ConnectedHandler {
        ConnectedHandler { callbacks: vec![] }
    }
}

pub struct DisconnectedHandler {
    pub callbacks: Vec<DisconnectedCallback>,
}

impl DisconnectedHandler {
    pub fn call(&mut self) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)()?;
        }
        Ok(())
    }
}

impl Handler for DisconnectedHandler {
    fn new() -> DisconnectedHandler {
        DisconnectedHandler { callbacks: vec![] }
    }
}

pub struct KeysHandler {
    pub callbacks: Vec<KeysCallback>,
}

impl KeysHandler {
    pub fn call(&mut self, se: &SpeedEditor, keys: &Vec<Key>) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(se, keys.clone())?;
        }
        Ok(())
    }
}

impl Handler for KeysHandler {
    fn new() -> KeysHandler {
        KeysHandler { callbacks: vec![] }
    }
}

pub struct KeyDownHandler {
    pub callbacks: Vec<KeyDownCallback>,
}

impl KeyDownHandler {
    pub fn call(&mut self, se: &SpeedEditor, key: Key) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(se, key)?;
        }
        Ok(())
    }
}

impl Handler for KeyDownHandler {
    fn new() -> KeyDownHandler {
        KeyDownHandler { callbacks: vec![] }
    }
}

pub struct KeyUpHandler {
    pub callbacks: Vec<KeyUpCallback>,
}

impl KeyUpHandler {
    pub fn call(&mut self, se: &SpeedEditor, key: Key) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(se, key)?;
        }
        Ok(())
    }
}

impl Handler for KeyUpHandler {
    fn new() -> KeyUpHandler {
        KeyUpHandler { callbacks: vec![] }
    }
}

pub struct JogHandler {
    pub callbacks: Vec<JogCallback>,
}

impl JogHandler {
    pub fn call(&mut self, se: &SpeedEditor, mode: u8, value: i32) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(se, mode, value)?;
        }
        Ok(())
    }
}

impl Handler for JogHandler {
    fn new() -> JogHandler {
        JogHandler { callbacks: vec![] }
    }
}

pub struct UnknownHandler {
    pub callbacks: Vec<UnknownCallback>,
}

impl UnknownHandler {
    pub fn call(&mut self, se: &SpeedEditor, data: &[u8]) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(se, data)?;
        }
        Ok(())
    }
}

impl Handler for UnknownHandler {
    fn new() -> UnknownHandler {
        UnknownHandler { callbacks: vec![] }
    }
}
