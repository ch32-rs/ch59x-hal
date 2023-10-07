use core::ptr;

const R8_CHIP_ID: u32 = 0x40001041;

pub fn get_chip_id() -> u8 {
    unsafe { ptr::read_volatile(R8_CHIP_ID as *const u8) }
}
