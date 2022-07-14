use num_enum::TryFromPrimitive;
use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd, TryFromPrimitive, Debug)]
pub enum Key {
    None = 0,

    SmartInsrt = 1,
    Appnd = 2,
    RiplOwr = 3,

    CloseUp = 4,
    PlaceOnTop = 5,
    SrcOwr = 6,

    In = 7,
    Out = 8,

    TrimIn = 9,
    TrimOut = 10,
    Roll = 11,

    SlipSrc = 12,
    SlipDest = 13,
    TransDur = 14,

    Cut = 15,
    Dis = 16,
    SmthCut = 17,

    Esc = 49,
    SyncBin = 31,
    AudioLevel = 44,
    FullView = 45,

    Trans = 34,
    Split = 47,
    Snap = 46,
    RiplDel = 43,

    Cam7 = 57,
    Cam8 = 58,
    Cam9 = 59,
    LiveOwr = 48,

    Cam4 = 54,
    Cam5 = 55,
    Cam6 = 56,
    VideoOnly = 37,

    Cam1 = 51,
    Cam2 = 52,
    Cam3 = 53,
    AudioOnly = 38,
    StopPlay = 60,

    Source = 26,
    Timeline = 27,

    Shtl = 28,
    Jog = 29,
    Scrl = 30,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
