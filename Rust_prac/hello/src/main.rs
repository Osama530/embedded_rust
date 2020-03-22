#![no_std]
#![no_main]

extern crate panic_halt;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main()->! {
    hprintln!("hay osama").unwrap();
    loop {

    }
}
