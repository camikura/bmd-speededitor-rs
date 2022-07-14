pub mod handler;
pub mod key;
pub mod key_led;

use chrono::{DateTime, Utc};
use hidapi::{HidDevice, HidError};
use std::{io::Read, rc::Rc, thread, time::Duration};
use strum::IntoEnumIterator;

use key::Key;
use key_led::KeyLed;

#[derive(Clone)]
pub struct SpeedEditor {
    pub device: Option<Rc<HidDevice>>,
    pub last_authenticated_at: Option<DateTime<Utc>>,
    pub current_keys: Vec<Key>,
    pub current_key_leds: Vec<KeyLed>,
    pub connected_handler: handler::ConnectedHandler,
    pub disconnected_handler: handler::DisconnectedHandler,
    pub keys_handler: handler::KeysHandler,
    pub key_down_handler: handler::KeyDownHandler,
    pub key_up_handler: handler::KeyUpHandler,
    pub jog_handler: handler::JogHandler,
    pub unknown_handler: handler::UnknownHandler,
}

#[derive(Debug)]
pub enum SpeedEditorError {
    HidApiError(hidapi::HidError),
    StdIoError(std::io::Error),
    AuthGetKbdChallengeError,
    AuthGetKbdResponseError,
    AuthGetKbdStatusError,
    CallbackExecutionError,
}

impl From<hidapi::HidError> for SpeedEditorError {
    fn from(e: HidError) -> Self {
        SpeedEditorError::HidApiError(e)
    }
}

impl From<std::io::Error> for SpeedEditorError {
    fn from(e: std::io::Error) -> Self {
        SpeedEditorError::StdIoError(e)
    }
}

impl SpeedEditor {
    const VID: u16 = 7899;
    const PID: u16 = 55822;
    const READ_TIMEOUT: i32 = 1000;
    const RECONNECT_INTERVAL: u64 = 100;
    const AUTH_INTERVAL: i64 = 30000;

    const AUTH_EVEN_TBL: [u64; 8] = [
        4242707987619187656,
        3069963097229903046,
        2352841328256802570,
        12646368222702737177,
        17018789593460232529,
        12706253227766860309,
        11978781369061872007,
        8438608961089703390,
    ];

    const AUTH_ODD_TBL: [u64; 8] = [
        4477338132788707294,
        2622620659002747676,
        11637077509869926595,
        7923852755392722584,
        8224257920127642516,
        4049197610885016386,
        18266591397768539273,
        7035737829027231430,
    ];

    const MASK: u64 = 12077075256910773232;

    fn rol8(&self, v: u64) -> u64 {
        ((v << 56) | (v >> 8)) & 18446744073709551615
    }

    fn rol8n(&self, mut v: u64, n: u64) -> u64 {
        for _ in 0..n {
            v = self.rol8(v);
        }
        v
    }

    /*
     * Authenticate module is taken from:
     * https://github.com/smunaut/blackmagic-misc
     * Copyright (C) 2021 Sylvain Munaut <tnt@246tNt.com>
     *
     * */
    pub fn auth_or_reconnect(&mut self) -> Result<(), SpeedEditorError> {
        if let Some(device) = self.device.clone() {
            self.auth(device)?;
        } else {
            self.connect()?;
        }

        Ok(())
    }
    pub fn auth(&mut self, device: Rc<HidDevice>) -> Result<(), SpeedEditorError> {
        let mut buf = [0; 8];
        let mut bytes = vec![0; 10];

        device.send_feature_report(&[0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0])?;
        bytes[0] = 0x6;

        let _ = device.get_feature_report(&mut bytes)?;
        if bytes[0] != 0x6 || bytes[1] != 0x0 {
            return Err(SpeedEditorError::AuthGetKbdChallengeError);
        }

        (&bytes[2..]).read_exact(&mut buf).unwrap();
        let challenge = u64::from_le_bytes(buf);

        device.send_feature_report(&[0x6, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0])?;
        let _ = device.get_feature_report(&mut bytes)?;
        if bytes[0] != 0x6 || bytes[1] != 0x2 {
            return Err(SpeedEditorError::AuthGetKbdResponseError);
        }

        let n = challenge & 7;
        let mut v = self.rol8n(challenge, n);
        let k: u64;
        if (v & 1) == ((120 >> n) & 1) {
            k = Self::AUTH_EVEN_TBL[n as usize];
        } else {
            v = v ^ self.rol8(v);
            k = Self::AUTH_ODD_TBL[n as usize];
        }

        let response = v ^ (self.rol8(v) & Self::MASK) ^ k;
        buf = response.to_le_bytes();

        bytes[1] = 0x3;
        for i in 0..8 {
            bytes[i + 2] = buf[i];
        }

        device.send_feature_report(bytes.as_slice())?;

        let _ = device.get_feature_report(&mut bytes)?;
        if bytes[0] != 0x6 || bytes[1] != 0x4 {
            return Err(SpeedEditorError::AuthGetKbdStatusError);
        }

        self.last_authenticated_at = Some(Utc::now());

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), SpeedEditorError> {
        loop {
            if let Some(device) = self.device.clone() {
                self.handle_loop(device)?;
            } else {
                self.connect()?;
            }
        }
    }

    pub fn handle_loop(&mut self, device: Rc<HidDevice>) -> Result<(), SpeedEditorError> {
        match self.last_authenticated_at {
            Some(last_authenticated_at) => {
                let elapsed_time = Utc::now() - last_authenticated_at;
                if elapsed_time.num_milliseconds() >= Self::AUTH_INTERVAL {
                    self.auth_or_reconnect()?;
                }
            }
            None => self.auth_or_reconnect()?,
        }

        let mut buf = [0; 64];
        match device.read_timeout(&mut buf, Self::READ_TIMEOUT) {
            Ok(len) => {
                if len > 0 {
                    self.process_events(&buf[..len])?;
                }
            }
            Err(_) => self.disconnect()?,
        }

        Ok(())
    }

    pub fn jog_event(&self, mode: u8, buf: &[u8]) -> Result<(), SpeedEditorError> {
        let mut data = [0; 4];
        (&buf[..]).read_exact(&mut data)?;
        let value = i32::from_le_bytes(data) / 360;
        (self.jog_handler)(&self.clone(), mode, value)
    }

    pub fn key_event(&mut self, buf: &[u8]) -> Result<(), SpeedEditorError> {
        let current_keys: Vec<Key> = buf
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .filter(|&(_, &v)| v > 0)
            .map(|(_, &v)| Key::try_from(v).unwrap())
            .collect();

        // Are you pressing 7 or more keys at the same time?
        if current_keys == self.current_keys {
            return Ok(());
        }

        let down_keys: Vec<Key> = current_keys
            .iter()
            .map(|&v| {
                if self.current_keys.iter().find(|&k| *k == v) == None {
                    v
                } else {
                    Key::None
                }
            })
            .filter(|&v| v > Key::None)
            .collect();

        let up_keys: Vec<Key> = self
            .current_keys
            .iter()
            .map(|&v| {
                if current_keys.iter().find(|&k| *k == v) == None {
                    v
                } else {
                    Key::None
                }
            })
            .filter(|&v| v > Key::None)
            .collect();

        self.current_keys = current_keys.to_owned();

        for k in down_keys {
            (self.key_down_handler)(&self.to_owned(), k)?;
        }
        for k in up_keys {
            (self.key_up_handler)(&self.to_owned(), k)?;
        }
        (self.keys_handler)(&self.to_owned(), self.current_keys.to_owned())?;

        Ok(())
    }

    pub fn unknown_event(&self, buf: &[u8]) -> Result<(), SpeedEditorError> {
        (self.unknown_handler)(&self.to_owned(), buf)
    }

    pub fn process_events(&mut self, buf: &[u8]) -> Result<(), SpeedEditorError> {
        match buf[0] {
            3 => {
                let mode = buf[1];
                self.jog_event(mode, &buf[2..])?
            }
            4 => self.key_event(&buf[1..])?,
            _ => self.unknown_event(buf)?,
        }

        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), SpeedEditorError> {
        self.device = None;
        (self.disconnected_handler)()
    }

    // Try to connect
    pub fn connect(&mut self) -> Result<(), SpeedEditorError> {
        let api = hidapi::HidApi::new()?;
        self.device = match api.open(SpeedEditor::VID, SpeedEditor::PID) {
            Ok(device) => Some(Rc::new(device)),
            Err(_) => None,
        };

        match self.device {
            Some(_) => {
                (self.connected_handler)(&self.clone())?;
            }
            None => {
                thread::sleep(Duration::from_millis(Self::RECONNECT_INTERVAL));
            }
        }

        Ok(())
    }

    pub fn add_key_led(&mut self, led: KeyLed) {
        self.current_key_leds.push(led);
    }

    pub fn remove_key_led(&mut self, led: KeyLed) {
        self.current_key_leds = self
            .current_key_leds
            .iter()
            .filter(|&i| *i != led)
            .map(|i| *i)
            .collect::<Vec<KeyLed>>();
    }

    pub fn set_all_key_leds(&mut self, on: bool) -> Result<(), SpeedEditorError> {
        for led in KeyLed::iter() {
            if on {
                self.add_key_led(led);
            } else {
                self.remove_key_led(led);
            }
        }

        self.light_key_leds()?;

        Ok(())
    }

    pub fn set_key_led(&mut self, led: KeyLed, on: bool) -> Result<(), SpeedEditorError> {
        if on {
            self.add_key_led(led);
        } else {
            self.remove_key_led(led);
        }

        self.light_key_leds()?;

        Ok(())
    }

    pub fn set_leds(&mut self, leds: Vec<KeyLed>, on: bool) -> Result<(), SpeedEditorError> {
        for led in leds {
            if on {
                self.add_key_led(led);
            } else {
                self.remove_key_led(led);
            }
        }

        self.light_key_leds()?;

        Ok(())
    }

    pub fn light_key_leds(&mut self) -> Result<(), SpeedEditorError> {
        if let Some(device) = self.device.clone() {
            let mut leds: i32 = 0;
            for i in self.current_key_leds.iter() {
                leds |= 1 << *i as i32;
            }

            let buf = leds.to_le_bytes();
            let mut data = [0x2, 0x0, 0x0, 0x0, 0x0, 0x0];
            data[0] = 0x2;
            for i in 0..4 {
                data[i + 1] = buf[i];
            }

            device.write(data.as_slice())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
