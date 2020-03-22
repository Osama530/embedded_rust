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

    let peripherals = stm32f3::stm32f303::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;
    let port_A = peripherals.GPIOA;
    let port_E = peripherals.GPIOE;
    
    rcc.ahbenr.write(|w| unsafe{
        w  
            .iopaen().set_bit()
            .iopeen().set_bit()
    });

    port_A.moder.write(|w| unsafe{
        w  
            .moder0().bits(0b10)
    });

    port_A.pupdr.write(|w| unsafe{
        w  
            .pupdr0().bits(0b10)
    });

    port_E.moder.write(|w| unsafe{
        w  
            .moder8().bits(0b01)

    });

    port_E.pupdr.write(|w| unsafe{
        w  
            .pupdr8().bits(0b00)

    });
        loop {
            // application logic
            let value = port_A.idr.read().idr0().bits();
                hprintln!("button state {} :",value);
            if value == true {
                port_E.bsrr.write(|w| w.bs8().set_bit());
                hprintln!("/t led 8 on ");
                }
            
            else {
                port_E.bsrr.write(|w| w.br8().set_bit());
                hprintln!("/t led 8 off ");
                }
            }
}   

