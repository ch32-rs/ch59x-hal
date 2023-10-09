//! Real time clock

use crate::{pac, with_safe_access};

// avoid cross 100-year, which is complex to handle leap year
const YEAR_OFFSET: u16 = 2020;

/// RTC Abstraction
pub struct Rtc;

impl Rtc {
    pub fn timestamp_since_epoch(&self) -> u32 {
        let rtc = unsafe { &*pac::RTC::PTR };

        let day = rtc.cnt_day.read().bits() & 0x3fff;
        let mut sec = rtc.cnt_2s.read().bits() * 2;
        if rtc.cnt_32k.read().bits() >= 0x8000 {
            sec += 1;
        }
        day * 86400 + (sec as u32)
    }

    pub fn counter_2s(&self) -> u16 {
        let rtc = unsafe { &*pac::RTC::PTR };
        rtc.cnt_2s.read().bits()
    }

    pub fn counter_day(&self) -> u16 {
        let rtc = unsafe { &*pac::RTC::PTR };
        (rtc.cnt_day.read().bits() & 0x3fff) as u16
    }

    // 32768
    pub fn counter_32k(&self) -> u16 {
        let rtc = unsafe { &*pac::RTC::PTR };
        rtc.cnt_32k.read().bits()
    }

    pub fn set_datatime(&mut self, t: DateTime) {
        let rtc = unsafe { &*pac::RTC::PTR };

        let mut days: u16 = (YEAR_OFFSET..t.year)
            .map(|y| if y % 4 == 0 { 366 } else { 365 })
            .sum();
        days += (1..=t.month - 1)
            .map(|m| days_in_month(m, t.year))
            .sum::<u16>();
        days += t.day as u16 - 1;

        let sec2 = ((t.hour as u16) % 24) * 1800 + (t.minute as u16) * 30 + (t.second as u16) / 2;
        let t32k: u16 = if t.second & 1 != 0 { 0x8000 } else { 0 };

        with_safe_access(|| unsafe {
            rtc.trig.write(|w| w.bits(days as u32));
            rtc.mode_ctrl.modify(|_, w| w.load_hi().set_bit());
        });
        // load_hi clear when loadded
        while rtc.mode_ctrl.read().load_hi().bit_is_set() {}
        let t = (sec2 as u32) << 16 | (t32k as u32);
        with_safe_access(|| unsafe {
            rtc.trig.write(|w| w.bits(t));
            rtc.mode_ctrl.modify(|_, w| w.load_lo().set_bit());
        });
    }

    pub fn now(&self) -> DateTime {
        let rtc = unsafe { &*pac::RTC::PTR };

        let mut days = (rtc.cnt_day.read().bits() & 0x3fff) as u16;
        let mut sec = rtc.cnt_2s.read().bits() * 2;
        if rtc.cnt_32k.read().bits() >= 0x8000 {
            sec += 1;
        }

        let mut year: u16 = YEAR_OFFSET;
        while days >= days_in_year(year) {
            days -= days_in_year(year);
            year += 1;
        }

        let mut month = 1;
        while days >= days_in_month(month, year) {
            days -= days_in_month(month, year);
            month += 1;
        }

        let hour = (sec / 3600) as u8;
        sec %= 3600;
        let minute = (sec / 60) as u8;
        sec %= 60;
        let second = (sec) as u8;

        DateTime {
            year,
            month: month,
            day: (days + 1) as u8,
            hour,
            minute,
            second,
        }
    }
}

fn days_in_year(year: u16) -> u16 {
    if year % 4 == 0 {
        366
    } else {
        365
    }
}
fn days_in_month(month: u8, year: u16) -> u16 {
    match month {
        1 => 31,
        2 => {
            if year % 4 == 0 {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct RtcConfig {}

/// Structure containing date and time information
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DateTime {
    /// 2020..2064
    pub year: u16,
    /// 1..12, 1 is January
    pub month: u8,
    /// 1..28,29,30,31 depending on month
    pub day: u8,
    //
    // pub day_of_week: DayOfWeek,
    /// 0..23
    pub hour: u8,
    /// 0..59
    pub minute: u8,
    /// 0..59
    pub second: u8,
}
