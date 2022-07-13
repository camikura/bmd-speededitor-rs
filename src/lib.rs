use chrono::{DateTime, Utc};
use hidapi::{HidDevice, HidError};
use std::io::Read;

pub type KeysCallback = fn(keys: Vec<u8>) -> Result<(), SpeedEditorError>;
pub type KeyDownCallback = fn(key: u8) -> Result<(), SpeedEditorError>;
pub type KeyUpCallback = fn(key: u8) -> Result<(), SpeedEditorError>;
pub type JogCallback = fn(value: i32) -> Result<(), SpeedEditorError>;
pub type UnknownCallback = fn(data: &[u8]) -> Result<(), SpeedEditorError>;

pub struct SpeedEditor {
    pub device: HidDevice,
    pub last_authenticated_at: Option<DateTime<Utc>>,
    pub current_keys: Vec<u8>,
    pub keys_callback: KeysCallback,
    pub key_down_callback: KeyDownCallback,
    pub key_up_callback: KeyUpCallback,
    pub jog_callback: JogCallback,
    pub unknown_callback: UnknownCallback,
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
    pub fn auth(&mut self) -> Result<(), SpeedEditorError> {
        let mut buf = [0; 8];
        let mut bytes = vec![0; 10];

        self.device
            .send_feature_report(&[0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0])?;

        bytes[0] = 0x6;

        let _ = self.device.get_feature_report(&mut bytes)?;
        if bytes[0] != 0x6 || bytes[1] != 0x0 {
            return Err(SpeedEditorError::AuthGetKbdChallengeError);
        }

        (&bytes[2..]).read_exact(&mut buf).unwrap();
        let challenge = u64::from_le_bytes(buf);

        self.device
            .send_feature_report(&[0x6, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0])?;
        let _ = self.device.get_feature_report(&mut bytes)?;
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

        self.device.send_feature_report(bytes.as_slice())?;

        let _ = self.device.get_feature_report(&mut bytes)?;
        if bytes[0] != 0x6 || bytes[1] != 0x4 {
            return Err(SpeedEditorError::AuthGetKbdStatusError);
        }

        self.last_authenticated_at = Some(Utc::now());

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), SpeedEditorError> {
        let mut buf = [0; 64];
        loop {
            match self.last_authenticated_at {
                Some(last_authenticated_at) => {
                    let elapsed_time = Utc::now() - last_authenticated_at;
                    if elapsed_time.num_milliseconds() >= Self::AUTH_INTERVAL {
                        self.auth()?;
                    }
                }
                None => self.auth()?,
            }

            let len = self.device.read_timeout(&mut buf, Self::READ_TIMEOUT)?;
            if len > 0 {
                if let Err(e) = self.process_events(&buf[..len]) {
                    eprintln!("{:?}", e);
                }
            }
        }
    }

    pub fn jog_event(&self, buf: &[u8]) -> Result<(), SpeedEditorError> {
        let mut data = [0; 4];
        (&buf[..]).read_exact(&mut data)?;
        let value = i32::from_le_bytes(data) / 360;
        (self.jog_callback)(value)
    }

    pub fn key_event(&mut self, buf: &[u8]) -> Result<(), SpeedEditorError> {
        let current_keys = buf
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .filter(|&(_, &v)| v > 0)
            .map(|(_, &v)| v)
            .collect::<Vec<u8>>();

        // Are you pressing 7 or more keys at the same time?
        if current_keys == self.current_keys {
            return Ok(());
        }

        let down_keys = current_keys
            .iter()
            .map(|&v| {
                if self.current_keys.iter().find(|&k| *k == v) == None {
                    v
                } else {
                    0
                }
            })
            .filter(|&v| v > 0)
            .collect::<Vec<u8>>();

        let up_keys = self
            .current_keys
            .iter()
            .map(|&v| {
                if current_keys.iter().find(|&k| *k == v) == None {
                    v
                } else {
                    0
                }
            })
            .filter(|&v| v > 0)
            .collect::<Vec<u8>>();

        self.current_keys = current_keys.clone();

        for k in down_keys {
            (self.key_down_callback)(k)?;
        }
        for k in up_keys {
            (self.key_up_callback)(k)?;
        }
        (self.keys_callback)(self.current_keys.to_owned())?;

        Ok(())
    }

    pub fn unknown_event(&self, buf: &[u8]) -> Result<(), SpeedEditorError> {
        (self.unknown_callback)(buf)
    }

    pub fn process_events(&mut self, buf: &[u8]) -> Result<(), SpeedEditorError> {
        match buf[0] {
            3 => self.jog_event(&buf[2..])?,
            4 => self.key_event(&buf[1..])?,
            _ => self.unknown_event(buf)?,
        }

        Ok(())
    }
}

pub fn new() -> Result<SpeedEditor, SpeedEditorError> {
    let api = hidapi::HidApi::new()?;
    let device = api.open(SpeedEditor::VID, SpeedEditor::PID)?;

    let se = SpeedEditor {
        device,
        last_authenticated_at: None,
        current_keys: Vec::default(),
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
