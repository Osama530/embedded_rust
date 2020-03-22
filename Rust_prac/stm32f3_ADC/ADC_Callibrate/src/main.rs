#![no_std]
#![no_main]

extern crate panic_semihosting ;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m_semihosting::hprint;


#[entry]
fn main() ->! {
    //initializing peripherals
    let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
    //calling rcc from library
    let rcc = peripherals.RCC;
    let mut adc = peripherals.ADC1;
    let adc12 = peripherals.ADC1_2;


    //ENABLING adc12 clock
    rcc.ahbenr.write(|w|
        w
            .adc12en().set_bit()
    );

    //setting SYSCLK not divided
    rcc.cfgr.write(|w| unsafe {
        w
            .hpre().bits(0b0111)
    });
    

    
    //enabling internal refrence voltage source 
    adc12.ccr.write(|w| 
        w 
            .vrefen().set_bit() //enabling internal refrence voltage source
            .ckmode().bits(0b01) //setting clock mode for adc as divided by 2
        );

    callibrat_adc(&mut adc);

     let cal_val = adc.calfact.read().calfact_s().bits();
     hprintln!("calibrated succesfully {} ",cal_val);



      loop {
            
            
        }    

            
}

fn callibrat_adc(adc: &mut stm32f3::stm32f303::ADC1 ){   
    adc.cr.write(|w| unsafe{
        w 
            .advregen().bits(00) //reset voltage regulator
            .advregen().bits(01) //enabling voltage regulator

    });

    //wait untill adc refrence voltage enable (10us)
    let mut count = 0;
    while count == 1000 {
        count+=1;
    } 

    adc.cr.write(|w| 
        w 
            .aden().clear_bit() //disable adc before callibration
            .adcaldif().clear_bit() //single-ended input conversions selected
            .adcal().set_bit() //start calibrating
    );

    hprintln!("calibrating");

    //read calibration status
    let cal_status = adc.cr.read().adcal().bits();
    hprintln!("calibrated status {} ",cal_status);

    //wait till calibration completes
    while cal_status != false {
        hprint!(".");
    }

    let cal_val = adc.calfact.read().calfact_s().bits();
    hprintln!("calibrated succesfully {} ",cal_val);
 }



     // //chennel selection
    // adc.sqr1.write(|w| unsafe{
    //     w
    //         .sq1().bits(0b10010) //1st in sequence and channle 18(0b10010)
    //         .l().bits(0000) //with length on sequence as one
    //     });
    // //setting sampling time
    // adc.smpr2.write(|w| unsafe{
    //     w
    //         .smp18().bits(0b000) //for channle 18, 15adc clocks = 2us
    // });
    // //wait 2usec


    // //selecting mode
    // adc.cfgr.write(|w| 
    //     w
    //         .cont().set_bit()
    // );

    
    
    // //initiating adc calibration
    // //callibrat_adc(&mut adc);

    // let value = adc.dr.read().rdata().bits();
    // hprintln!("value = {} ",value);
  

    // hprintln!("hello world !").unwrap();