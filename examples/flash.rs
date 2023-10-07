#![no_std]
#![no_main]

use ch59x_hal as hal;
use hal::pac;
use panic_halt as _;



extern "C" {
    pub fn FLASH_EEPROM_CMD(cmd: u8, star_addr: u32, buffer: *const u8, len: u32) -> u32;
}


#[riscv_rt::entry]
fn main() -> ! {
    // LED PA4, PB23

    let p = unsafe { pac::Peripherals::steal() };

    unsafe {
        p.GPIO.pa_pd_drv.modify(|_, w| w.pa_pd_drv().bits(1 << 4));
        p.GPIO.pa_dir.modify(|_, w| w.pa_dir().bits(1 << 4));

        p.GPIO.pb_pd_drv.modify(|_, w| w.pb_pd_drv().bits(1 << 23));
        p.GPIO.pb_dir.modify(|_, w| w.pb_dir().bits(1 << 23));

        p.GPIO
            .pb_out
            .modify(|r, w| w.pb_out().bits(r.pb_out().bits() ^ (1 << 23)));
    }

    loop {
        unsafe {
            p.GPIO
                .pa_out
                .modify(|r, w| w.pa_out().bits(r.pa_out().bits() ^ (1 << 4)));
            p.GPIO
                .pb_out
                .modify(|r, w| w.pb_out().bits(r.pb_out().bits() ^ (1 << 23)));
            riscv::asm::delay(100000);
        }
    }
}
