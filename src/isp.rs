//! The libISP592 library.

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum RomCmd {
    FlashRomStartIo = 0x00, // start FlashROM I/O, without parameter
    FlashRomSwReset = 0x04, // software reset FlashROM, without parameter
    GetRomInfo = 0x06,      // get information from FlashROM, parameter @Address,Buffer
    GetUniqueId = 0x07,     // get 64 bit unique ID, parameter @Buffer
    FlashRomPwrDown = 0x0D, // power-down FlashROM, without parameter
    FlashRomPwrUp = 0x0C,   // power-up FlashROM, without parameter
    FlashRomLock = 0x08, // lock(protect)/unlock FlashROM data block, return 0 if success, parameter @StartAddr
    // StartAddr: 0=unlock all, 1=lock boot code, 3=lock all code and data
    EepromErase = 0x09, // erase Data-Flash block, return 0 if success, parameter @StartAddr,Length
    EepromWrite = 0x0A, // write Data-Flash data block, return 0 if success, parameter @StartAddr,Buffer,Length
    EepromRead = 0x0B,  // read Data-Flash data block, parameter @StartAddr,Buffer,Length
    FlashRomErase = 0x01, // erase FlashROM block, return 0 if success, parameter @StartAddr,Length
    FlashRomWrite = 0x02, // write FlashROM data block, minimal block is dword, return 0 if success, parameter @StartAddr,Buffer,Length
    FlashRomVerify = 0x03, // read FlashROM data block, minimal block is dword, return 0 if success, parameter @StartAddr,Buffer,Length
}

/*
#define ROM_CFG_MAC_ADDR	0x7F018			// address for MAC address information
#define ROM_CFG_BOOT_INFO	0x7DFF8			// address for BOOT information
 */
extern "C" {
    pub fn FLASH_EEPROM_CMD(cmd: u8, star_addr: u32, buffer: *const u8, len: u32) -> u32;
}

static mut TEST_BUF: [u8; 1024] = [0; 1024];

/// get 64 bit unique ID
pub fn get_unique_id() -> [u32; 2] {
    let mut id = [0u32; 2];
    unsafe {
        FLASH_EEPROM_CMD(RomCmd::GetUniqueId as u8, 0, TEST_BUF.as_mut_ptr() as _, 0);
        TEST_BUF.chunks_exact(4).for_each(|chunk| {
            id[0] = id[0].rotate_left(8) | u32::from(chunk[0]);
            id[1] = id[1].rotate_left(8) | u32::from(chunk[1]);
        });
    }
    id
}

pub fn get_mac_addr() -> [u8; 6] {
    const ROM_CFG_MAC_ADDR: u32 = 0x7F018;
    // let mut mac = [0u8; 6];
    unsafe {
        FLASH_EEPROM_CMD(
            RomCmd::GetRomInfo as u8,
            ROM_CFG_MAC_ADDR,
            TEST_BUF.as_mut_ptr(),
            0,
        );
    }
    [0u8; 6]
}

pub fn get_boot_info() -> [u8; 8] {
    const ROM_CFG_BOOT_INFO: u32 = 0x7DFF8;
    let mut boot_info = [0u8; 8];
    unsafe {
        FLASH_EEPROM_CMD(
            RomCmd::GetRomInfo as u8,
            ROM_CFG_BOOT_INFO,
            boot_info.as_mut_ptr(),
            0,
        );
    }
    boot_info
}

pub fn read_flash_rom(start_addr: u32, len: u32) -> &'static [u8] {
    unsafe {
        FLASH_EEPROM_CMD(
            RomCmd::FlashRomVerify as u8,
            start_addr,
            TEST_BUF.as_mut_ptr(),
            len,
        );
        &TEST_BUF[..128]
    }
}
