#![no_std]
#![no_main]

extern crate panic_semihosting ;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() ->! {
    let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;
    let port_E = peripherals.GPIOE;
    rcc.ahbenr.write(|w| w.iopeen().bit(true));
    port_E.moder.write(|w| unsafe{
        w
            .moder8().bits(0b01)
    });
    port_E.pupdr.write(|w| unsafe{ 
        w
            .pupdr8().bits(0b00)
    });

    port_E.bsrr.write(|w| w.bs8().set_bit());

    
    hprintln!("hello world !").unwrap();
        loop {
            // application logic
        }
}
