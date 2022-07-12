use hidapi::{HidDevice, HidError};
use std::io::Read;

pub struct SpeedEditor {
    pub device: HidDevice,
}

#[derive(Debug)]
pub enum SpeedEditorError {
    HidApiError(hidapi::HidError),
    AuthGetKbdChallengeError,
    AuthGetKbdResponseError,
    AuthGetKbdStatusError,
}

impl From<hidapi::HidError> for SpeedEditorError {
    fn from(e: HidError) -> Self {
        SpeedEditorError::HidApiError(e)
    }
}

impl SpeedEditor {
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
    pub fn auth(&self) -> Result<(), SpeedEditorError> {
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

        Ok(())
    }

    pub fn run(&self) -> Result<(), HidError> {
        let mut buf = [0; 64];
        loop {
            let len = self.device.read(&mut buf)?;
            println!("{:?}", &buf[..len]);
        }
    }
}

pub fn new() -> Result<SpeedEditor, SpeedEditorError> {
    let api = hidapi::HidApi::new()?;
    let device = api.open(7899, 55822)?;

    let se = SpeedEditor { device };
    se.auth()?;

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
