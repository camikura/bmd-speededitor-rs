use num_enum::TryFromPrimitive;
use std::fmt;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd, TryFromPrimitive, Debug, EnumIter)]
pub enum KeyLed {
    CloseUp = 0,

    Cut = 1,
    Dis = 2,
    SmthCut = 3,

    Trans = 4,
    Snap = 5,

    Cam7 = 6,
    Cam8 = 7,
    Cam9 = 8,
    LiveOwr = 9,

    Cam4 = 10,
    Cam5 = 11,
    Cam6 = 12,
    VideoOnly = 13,

    Cam1 = 14,
    Cam2 = 15,
    Cam3 = 16,
    AudioOnly = 17,
}

impl fmt::Display for KeyLed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
