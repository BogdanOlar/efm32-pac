#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipinsel {
    #[doc = "Pin 0."]
    Pin0 = 0x0,
    #[doc = "Pin 1."]
    Pin1 = 0x01,
    #[doc = "Pin 2."]
    Pin2 = 0x02,
    #[doc = "Pin 3."]
    Pin3 = 0x03,
}
impl Extipinsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipinsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipinsel {
    #[inline(always)]
    fn from(val: u8) -> Extipinsel {
        Extipinsel::from_bits(val)
    }
}
impl From<Extipinsel> for u8 {
    #[inline(always)]
    fn from(val: Extipinsel) -> u8 {
        Extipinsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extipsel {
    #[doc = "Port A group selected for external interrupt 0."]
    Porta = 0x0,
    #[doc = "Port B group selected for external interrupt 0."]
    Portb = 0x01,
    #[doc = "Port C group selected for external interrupt 0."]
    Portc = 0x02,
    #[doc = "Port D group selected for external interrupt 0."]
    Portd = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Port F group selected for external interrupt 0."]
    Portf = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Extipsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extipsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extipsel {
    #[inline(always)]
    fn from(val: u8) -> Extipsel {
        Extipsel::from_bits(val)
    }
}
impl From<Extipsel> for u8 {
    #[inline(always)]
    fn from(val: Extipsel) -> u8 {
        Extipsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Input disabled. Pullup if DOUT is set."]
    Disabled = 0x0,
    #[doc = "Input enabled. Filter if DOUT is set."]
    Input = 0x01,
    #[doc = "Input enabled. DOUT determines pull direction."]
    Inputpull = 0x02,
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 0x03,
    #[doc = "Push-pull output."]
    Pushpull = 0x04,
    #[doc = "Push-pull using alternate control."]
    Pushpullalt = 0x05,
    #[doc = "Wired-or output."]
    Wiredor = 0x06,
    #[doc = "Wired-or output with pull-down."]
    Wiredorpulldown = 0x07,
    #[doc = "Open-drain output."]
    Wiredand = 0x08,
    #[doc = "Open-drain output with filter."]
    Wiredandfilter = 0x09,
    #[doc = "Open-drain output with pullup."]
    Wiredandpullup = 0x0a,
    #[doc = "Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 0x0b,
    #[doc = "Open-drain output using alternate control."]
    Wiredandalt = 0x0c,
    #[doc = "Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 0x0d,
    #[doc = "Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 0x0e,
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 0x0f,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swvloc {
    #[doc = "Location 0."]
    Loc0 = 0x0,
    #[doc = "Location 1."]
    Loc1 = 0x01,
    #[doc = "Location 2."]
    Loc2 = 0x02,
    #[doc = "Location 3."]
    Loc3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Swvloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swvloc {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swvloc {
    #[inline(always)]
    fn from(val: u8) -> Swvloc {
        Swvloc::from_bits(val)
    }
}
impl From<Swvloc> for u8 {
    #[inline(always)]
    fn from(val: Swvloc) -> u8 {
        Swvloc::to_bits(val)
    }
}
