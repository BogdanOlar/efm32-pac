#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlCmoa {
    #[doc = "A single clock cycle pulse is generated on output."]
    Pulse = 0x0,
    #[doc = "Toggle output on compare match."]
    Toggle = 0x01,
    #[doc = "Clear output on compare match."]
    Clear = 0x02,
    #[doc = "Set output on compare match."]
    Set = 0x03,
}
impl CcCtrlCmoa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlCmoa {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlCmoa {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlCmoa {
        CcCtrlCmoa::from_bits(val)
    }
}
impl From<CcCtrlCmoa> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlCmoa) -> u8 {
        CcCtrlCmoa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlIcedge {
    #[doc = "Rising edges detected."]
    Rising = 0x0,
    #[doc = "Falling edges detected."]
    Falling = 0x01,
    #[doc = "Both edges detected."]
    Both = 0x02,
    #[doc = "No edge detection, signal is left as it is."]
    None = 0x03,
}
impl CcCtrlIcedge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlIcedge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlIcedge {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlIcedge {
        CcCtrlIcedge::from_bits(val)
    }
}
impl From<CcCtrlIcedge> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlIcedge) -> u8 {
        CcCtrlIcedge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlMode {
    #[doc = "Compare/Capture channel turned off."]
    Off = 0x0,
    #[doc = "Input capture."]
    Inputcapture = 0x01,
    #[doc = "Output compare."]
    Outputcompare = 0x02,
    _RESERVED_3 = 0x03,
}
impl CcCtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlMode {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlMode {
        CcCtrlMode::from_bits(val)
    }
}
impl From<CcCtrlMode> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlMode) -> u8 {
        CcCtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcCtrlPrssel {
    #[doc = "PRS Channel 0 selected as input."]
    Prsch0 = 0x0,
    #[doc = "PRS Channel 1 selected as input."]
    Prsch1 = 0x01,
    #[doc = "PRS Channel 2 selected as input."]
    Prsch2 = 0x02,
    #[doc = "PRS Channel 3 selected as input."]
    Prsch3 = 0x03,
    #[doc = "PRS Channel 4 selected as input."]
    Prsch4 = 0x04,
    #[doc = "PRS Channel 5 selected as input."]
    Prsch5 = 0x05,
    #[doc = "PRS Channel 6 selected as input."]
    Prsch6 = 0x06,
    #[doc = "PRS Channel 7 selected as input."]
    Prsch7 = 0x07,
    #[doc = "PRS Channel 8 selected as input."]
    Prsch8 = 0x08,
    #[doc = "PRS Channel 9 selected as input."]
    Prsch9 = 0x09,
    #[doc = "PRS Channel 10 selected as input."]
    Prsch10 = 0x0a,
    #[doc = "PRS Channel 11 selected as input."]
    Prsch11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CcCtrlPrssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcCtrlPrssel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcCtrlPrssel {
    #[inline(always)]
    fn from(val: u8) -> CcCtrlPrssel {
        CcCtrlPrssel::from_bits(val)
    }
}
impl From<CcCtrlPrssel> for u8 {
    #[inline(always)]
    fn from(val: CcCtrlPrssel) -> u8 {
        CcCtrlPrssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cntpresc {
    #[doc = "CLKCNT = LFECLKRTCC/1."]
    Div1 = 0x0,
    #[doc = "CLKCNT = LFECLKRTCC/2."]
    Div2 = 0x01,
    #[doc = "CLKCNT = LFECLKRTCC/4."]
    Div4 = 0x02,
    #[doc = "CLKCNT = LFECLKRTCC/8."]
    Div8 = 0x03,
    #[doc = "CLKCNT = LFECLKRTCC/16."]
    Div16 = 0x04,
    #[doc = "CLKCNT = LFECLKRTCC/32."]
    Div32 = 0x05,
    #[doc = "CLKCNT = LFECLKRTCC/64."]
    Div64 = 0x06,
    #[doc = "CLKCNT = LFECLKRTCC/128."]
    Div128 = 0x07,
    #[doc = "CLKCNT = LFECLKRTCC/256."]
    Div256 = 0x08,
    #[doc = "CLKCNT = LFECLKRTCC/512."]
    Div512 = 0x09,
    #[doc = "CLKCNT = LFECLKRTCC/1024."]
    Div1024 = 0x0a,
    #[doc = "CLKCNT = LFECLKRTCC/2048."]
    Div2048 = 0x0b,
    #[doc = "CLKCNT = LFECLKRTCC/4096."]
    Div4096 = 0x0c,
    #[doc = "CLKCNT = LFECLKRTCC/8192."]
    Div8192 = 0x0d,
    #[doc = "CLKCNT = LFECLKRTCC/16384."]
    Div16384 = 0x0e,
    #[doc = "CLKCNT = LFECLKRTCC/32768."]
    Div32768 = 0x0f,
}
impl Cntpresc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cntpresc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cntpresc {
    #[inline(always)]
    fn from(val: u8) -> Cntpresc {
        Cntpresc::from_bits(val)
    }
}
impl From<Cntpresc> for u8 {
    #[inline(always)]
    fn from(val: Cntpresc) -> u8 {
        Cntpresc::to_bits(val)
    }
}
