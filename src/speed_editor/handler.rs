use super::{Key, SpeedEditorResult};

pub trait Handler {
    fn new() -> Self;
}

pub struct ConnectedHandler {
    pub callbacks: Vec<Box<dyn FnMut() -> SpeedEditorResult + Sync + Send>>,
}

impl ConnectedHandler {
    pub fn call(&mut self) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)()?;
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
    pub callbacks: Vec<Box<dyn FnMut() -> SpeedEditorResult + Sync + Send>>,
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
    pub callbacks: Vec<Box<dyn FnMut(Vec<Key>) -> SpeedEditorResult + Sync + Send>>,
}

impl KeysHandler {
    pub fn call(&mut self, keys: &Vec<Key>) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(keys.clone())?;
        }
        Ok(())
    }
}

impl Handler for KeysHandler {
    fn new() -> KeysHandler {
        KeysHandler { callbacks: vec![] }
    }
}

pub struct KeyHandler {
    pub callbacks: Vec<Box<dyn FnMut(Key, bool) -> SpeedEditorResult + Sync + Send>>,
}

impl KeyHandler {
    pub fn call(&mut self, key: Key, down: bool) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(key, down)?;
        }
        Ok(())
    }
}

impl Handler for KeyHandler {
    fn new() -> KeyHandler {
        KeyHandler { callbacks: vec![] }
    }
}

pub struct KeyDownHandler {
    pub callbacks: Vec<Box<dyn FnMut(Key) -> SpeedEditorResult + Sync + Send>>,
}

impl KeyDownHandler {
    pub fn call(&mut self, key: Key) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(key)?;
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
    pub callbacks: Vec<Box<dyn FnMut(Key) -> SpeedEditorResult + Sync + Send>>,
}

impl KeyUpHandler {
    pub fn call(&mut self, key: Key) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(key)?;
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
    pub callbacks: Vec<Box<dyn FnMut(u8, i32) -> SpeedEditorResult + Sync + Send>>,
}

impl JogHandler {
    pub fn call(&mut self, mode: u8, value: i32) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(mode, value)?;
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
    pub callbacks: Vec<Box<dyn FnMut(&[u8]) -> SpeedEditorResult + Sync + Send>>,
}

impl UnknownHandler {
    pub fn call(&mut self, data: &[u8]) -> SpeedEditorResult {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(data)?;
        }
        Ok(())
    }
}

impl Handler for UnknownHandler {
    fn new() -> UnknownHandler {
        UnknownHandler { callbacks: vec![] }
    }
}
