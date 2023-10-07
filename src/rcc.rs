use fugit::{HertzU32 as Hertz, RateExtU32};

use crate::pac::SYSCTL;

// No HSI
const HSE_FREQUENCY: Hertz = Hertz::from_raw(32_000_000);

pub struct CK32KConfig {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockSrc {
    // CK32K from LSI
    LSI,
    // CK32K from X32K, PA10, PA11
    LSE,
    // CK32M from HSE
    HSE,
    // CK32M from PLL
    PLL,
}

pub struct Config {
    pub mux: ClockSrc,
    // div for HSE and PLL, 2 to 32(0b0)
    pub div: u8,
}

impl Config {
    pub fn use_lsi() -> Self {
        Self {
            mux: ClockSrc::LSI,
            div: 0b00101,
        }
    }

    pub fn use_lse() -> Self {
        Self {
            mux: ClockSrc::LSE,
            div: 0b00101,
        }
    }

    pub fn pll_60mhz() -> Self {
        Self {
            mux: ClockSrc::PLL,
            div: 8, // 480 / 8 = 60
        }
    }

    pub fn pll_80mhz() -> Self {
        Self {
            mux: ClockSrc::PLL,
            div: 6, // 480 / 6 = 80
        }
    }
}
