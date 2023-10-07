use ch59x::ch59x::SYS;
use fugit::{HertzU32 as Hertz};

use crate::{pac::SYSCTL, with_safe_access};

// No HSI
const HSE_FREQUENCY: Hertz = Hertz::from_raw(32_000_000);
const PLL_FREQUENCY: Hertz = Hertz::from_raw(480_000_000);

/// 32K clock source
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Clock32KSrc {
    #[default]
    LSI,
    LSE,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ClockSrc {
    // CK32K
    Clock32K,
    // CK32M from HSE, then div
    HSE(u8),
    // CK32M from PLL, then div
    PLL(u8),
}

impl Default for ClockSrc {
    fn default() -> Self {
        Self::PLL(8)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Config {
    pub clock32ksrc: Clock32KSrc,
    pub mux: ClockSrc,
}

impl Config {
    pub fn clock_source_lsi() -> Self {
        Self {
            mux: ClockSrc::Clock32K,
            clock32ksrc: Clock32KSrc::LSI,
        }
    }

    pub fn pll_60mhz() -> Self {
        Self {
            mux: ClockSrc::PLL(8),
            ..Default::default()
        }
    }

    pub fn pll_80mhz() -> Self {
        Self {
            mux: ClockSrc::PLL(8),
            ..Default::default()
        }
    }

    pub fn freeze(self) -> Clocks {
        let sysctl = unsafe { &*SYSCTL::PTR };
        let sys = unsafe { &*SYS::PTR };
        with_safe_access(|| unsafe {
            sysctl
                .pll_config
                .modify(|r, w| w.pll_cfg_dat().bits(r.pll_cfg_dat().bits() & !(1 << 5)));
        });
        let hclk = match self.mux {
            ClockSrc::HSE(div) => {
                with_safe_access(|| unsafe {
                    sys.clk_sys_cfg.write(|w| {
                        w.pll_pwr_en()
                            .set_bit()
                            .xt_32m_pwr_en()
                            .set_bit()
                            .clk_sys_mod()
                            .bits(0b10)
                            .clk_pll_div()
                            .bits(div)
                    });
                    riscv::asm::nop();
                    riscv::asm::nop();
                    riscv::asm::nop();
                    riscv::asm::nop();
                });
                with_safe_access(|| unsafe {
                    sysctl.flash_cfg.modify(|_, w| w.bits(0x51));
                });
                Hertz::from_raw(HSE_FREQUENCY.to_Hz() / (div as u32))
            }
            ClockSrc::PLL(div) => {
                with_safe_access(|| unsafe {
                    sys.clk_sys_cfg.write(|w| {
                        w.pll_pwr_en()
                            .set_bit()
                            .xt_32m_pwr_en()
                            .set_bit()
                            .clk_sys_mod()
                            .bits(0b01)
                            .clk_pll_div()
                            .bits(div)
                    });
                    riscv::asm::nop();
                    riscv::asm::nop();
                    riscv::asm::nop();
                    riscv::asm::nop();
                });
                with_safe_access(|| unsafe {
                    sysctl.flash_cfg.modify(|_, w| w.bits(0x52));
                });
                Hertz::from_raw(PLL_FREQUENCY.to_Hz() / (div as u32))
            }
            _ => unimplemented!("CK32K not implemented"),
        };
        with_safe_access(|| unsafe {
            sysctl
                .pll_config
                .modify(|r, w| w.pll_cfg_dat().bits(r.pll_cfg_dat().bits() | (1 << 7)));
        });
        Clocks { hclk }
    }
}

pub struct Clocks {
    pub hclk: Hertz,
}
