// #![no_std]
// #![no_main]

// extern crate panic_semihosting ;
// extern crate cortex_m_rt;
// extern crate cortex_m_semihosting;

// use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;

// // use `main` as the entry point of this application
// // `main` is not allowed to return


// #[entry]
// fn main() ->! {
//     //initializing peripherals
//     let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
//     //calling rcc from library
//     let rcc = peripherals.RCC;
//     //initializing port_c
//     let port_c = peripherals.GPIOC;
//     let adc = peripherals.ADC1;
    
//     //ADC clock and port_c clocd enable (sec:9.4.6)
//     rcc.ahbenr.write(|w|
//         w
//             .iopcen().set_bit()
//             .adc12en().set_bit()
           
//     );
    
//         //seting pin as analoge
//         port_c.moder.write(|w| 
//             w
//                 .moder1().bits(0b11)
//         );
//         //setting pin as no pullUp/pullDown
//         port_c.pupdr.write(|w| unsafe{ 
//             w
//                 .pupdr1().bits(0b00)
//         });
    
//     //pll clock enable
//     rcc.cr.write(|w| w.pllon().set_bit());
//     //setting clock frequency
//     rcc.cfgr2.write(|w| unsafe{
//         w
//             .adc12pres().bits(0b10100)//PLL divided by 8)

//     });

//     //configuring adc mode and resolution of opration
//     adc.cfgr.write(|w|
//         w
//            .cont().set_bit()//continuase mode
//            .res().bits(0b10)//12 bit resolution
//            .align().clear_bit()//right align
//        );

//     //settting adc channel (channel 7 
//     adc.sqr2.write(|w| unsafe{
//         w
//             .sq7().bits(0b1100)
//     });
//     //settinhg channel sequence length = 1
//     adc.sqr1.write(|w| 
//         w
//             .l().bits(0b1)
//     );
    
//     //setting sampling time (0b011 = 7.5 clock cycles)
//     adc.smpr1.write(|w| w.smp7().bits(0b011));



//     if let cfg_mode = adc.cfgr.read().cont().bit_is_set() {
//     hprintln!("continuos mode status = {}",cfg_mode); }
    
//     let res = adc.cfgr.read().res().bits();
//         hprintln!("adc resolution = {}",res); 
//     //enable the adc and start conversion
//     adc.cr.write(|w|
//          w
//             .aden().set_bit() //enable adc
            
//         );
//     rcc.cr.read().hserdy().bit_is_clear();

//     //wait
//     let mut count = 0;
//     while count != 1000 {
//         count+=1;
//     }
//     //start conversion
//     adc.cr.write(|w| w.adstart().set_bit() );
    
    
//     let adc_en_state = adc.cr.read().aden().bits();
//     hprintln!("adc enabled = {}",adc_en_state);
//     let conversion_status = adc.cr.read().adstart().bits();
//     hprintln!("conversion start = {}",conversion_status);


//     let status = adc.isr.read().eoc().bits() as u8;
//                 hprintln!("eoc status : {:X}",status);
    




//     //port_E.bsrr.write(|w| w.bs8().set_bit());


//     hprintln!("hello world !").unwrap();
//         loop {
            
//             if  adc.isr.read().eoc().bit_is_set() {
//                 let data = adc.dr.read().bits();
//                 let voltage = (data * 3300)/4096;
//                 hprintln!("adc value : {}---{}",data,voltage);
//             }
            
//         }    
//             //port_E.bsrr.write(|w| w.br8().set_bit());
//             //delay(100);
            
// }


// #![no_std]
// #![no_main]

// extern crate panic_semihosting ;
// extern crate cortex_m_rt;
// extern crate cortex_m_semihosting;

// use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;

// // use `main` as the entry point of this application
// // `main` is not allowed to return

// fn port_config(port_c : &mut stm32f3::stm32f303::GPIOC ){
//     //seting pin as analoge
//     port_c.moder.write(|w| 
//         w
//             .moder1().bits(0b11)
//     );
//         //setting pin as no pullUp/pullDown
//     port_c.pupdr.write(|w| unsafe{ 
//         w
//             .pupdr1().bits(0b00)
//     });
//             }

// #[entry]
// fn main() ->! {
//     //initializing peripherals
//     let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
//     //calling rcc from library
//     let rcc = peripherals.RCC;
//     //initializing port_c
//     let mut port_c = peripherals.GPIOC;

    
//     //port_c clocd enable (sec:9.4.6)
//     rcc.ahbenr.write(|w|
//         w
//             .iopcen().set_bit()

           
//     );
//     port_config(&mut port_c);
    


//     hprintln!("hello world !").unwrap();
//         loop {
//             let mut value = port_c.idr.read().idr1().bits();
//             hprintln!("port_c pin 1 value = {} ",value).unwrap();

            
//         }    

            
// }

// //callibration
// #![no_std]
// #![no_main]

// extern crate panic_semihosting ;
// extern crate cortex_m_rt;
// extern crate cortex_m_semihosting;

// use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;
// use cortex_m_semihosting::hprint;


// // use `main` as the entry point of this application
// // `main` is not allowed to return


// #[entry]
// fn main() ->! {
//     //initializing peripherals
//     let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
//     //calling rcc from library
//     let rcc = peripherals.RCC;
//     let mut adc = peripherals.ADC1;

    
//     //initiating adc calibration
//     callibrat_adc(&mut adc);


//     let adc_cal_valu = adc.calfact.read().bits();
//     hprintln!("{}",adc_cal_valu);    

//     hprintln!("hello world !").unwrap();
//         loop {
            
            
//         }    

            
// }

// fn callibrat_adc(adc: &mut stm32f3::stm32f303::ADC1 ){   
//     adc.cr.write(|w| unsafe{
//         w 
//             .advregen().bits(00) //reset voltage regulator
//             .advregen().bits(01) //enabling voltage regulator

//     });
//     //wait untill adc refrence voltage enable (10us)
//     let mut count = 0;
//     while count == 1000 {
//         count+=1;
//     } 

//     adc.cr.write(|w| 
//         w 
//             .aden().clear_bit() //disable adc before callibration
//             .adcaldif().clear_bit() //single-ended input conversions selected
//             .adcal().set_bit() //start calibrating
//     );
//     hprintln!("calibrating");
//     while !adc.cr.read().adcal().bit_is_clear() {
//         hprint!(".");
//     }
//     hprintln!("calibrated succesfully");
//  }


// //internal temprature sensor reading
// #![no_std]
// #![no_main]

// extern crate panic_semihosting ;
// extern crate cortex_m_rt;
// extern crate cortex_m_semihosting;

// use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;
// use cortex_m_semihosting::hprint;


// // use `main` as the entry point of this application
// // `main` is not allowed to return


// #[entry]
// fn main() ->! {
//     //initializing peripherals
//     let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
//     //calling rcc from library
//     let rcc = peripherals.RCC;
//     let mut adc = peripherals.ADC1;
//     let adc12 = peripherals.ADC1_2;


//     //ENABLING adc12 clock
//     rcc.ahbenr.write(|w|
//         w
//             .adc12en().set_bit()
//     );
//     //end of conversion interupt enabling
//     adc.ier.write(|w|
//         w 
//             .eocie().set_bit()
//     );

//     //setting SYSCLK not divided
//     rcc.cfgr.write(|w| unsafe {
//         w
//             .hpre().bits(0b0111)
//     });
    

    
//     //enabling internal refrence voltage source 
//     adc12.ccr.write(|w| 
//         w 
//             .vrefen().set_bit() //enabling internal refrence voltage source
//             .ckmode().bits(0b10) //setting clock mode for adc as divided by 2
//         );

//     callibrat_adc(&mut adc);

//     //enabling adc
//     adc.cr.write(|w|
//         w
//             .aden().set_bit()
//     );

//     //enabling internal temp. sensor and adc
//     adc12.ccr.write(|w|
//         w
//             .tsen().set_bit()
//     );
    
//     //chennel selection
//     adc.sqr1.write(|w| unsafe{
//         w
//             .sq1().bits(0b10000) //1st in sequence and channle 16(0b10000)
//             .l().bits(0b0000) //with length on sequence as one
//         });
//     //setting sampling time
//     adc.smpr2.write(|w| unsafe{
//         w
//             .smp16().bits(0b001) //for channle 16, 
//     });

//     //selecting mode
//     adc.cfgr.write(|w| 
//         w
//             .cont().set_bit()
//             .exten().bits(0b01)
//     );

//     //start coverting
//     adc.cr.write(|w|
//         w
//             .adstart().set_bit()
//     );

    
//     let end_status = adc.isr.read().eoc().bits();
//     while end_status == true {
//         let value = adc.dr.read().rdata().bits();
//         hprintln!("Temprature = {} ",value);
//     }
  
    

//     // hprintln!("hello world !").unwrap();


//      loop {
//         while end_status == true {
//             let value = adc.dr.read().rdata().bits();
//             hprintln!("Temprature = {} ",value);
//         }
            
//         }    

            
// }

// fn callibrat_adc(adc: &mut stm32f3::stm32f303::ADC1 ){   
//     adc.cr.write(|w| unsafe{
//         w 
//             .advregen().bits(00) //reset voltage regulator
//             .advregen().bits(01) //enabling voltage regulator

//     });

//     //wait untill adc refrence voltage enable (10us)
//     let mut count = 0;
//     while count == 1000 {
//         count+=1;
//     } 

//     adc.cr.write(|w| 
//         w 
//             .aden().clear_bit() //disable adc before callibration
//             .adcaldif().clear_bit() //single-ended input conversions selected
//             .adcal().set_bit() //start calibrating
//     );

//     hprintln!("calibrating");

//     //read calibration status
//     let cal_status = adc.cr.read().adcal().bits();
//     hprintln!("calibrated status {} ",cal_status);

//     //wait till calibration completes
//     while cal_status != false {
//         hprint!(".");
//     }

//     let cal_val = adc.calfact.read().calfact_s().bits();
//     hprintln!("calibrated succesfully {} ",cal_val);
//  }

 //internal votage source mesurement
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
     
// //Delay
// #![no_std]
// #![no_main]

// extern crate panic_semihosting ;
// extern crate cortex_m_rt;
// extern crate cortex_m_semihosting;

// use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;
// use cortex_m_semihosting::hprint;


// // use `main` as the entry point of this application
// // `main` is not allowed to return


// #[entry]
// fn main() ->! {
//     //initializing peripherals
//     let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
//     //calling rcc from library
//     let rcc = peripherals.RCC;
//     let tim6 = peripherals.TIM6;

//     delay_init(&tim6, &rcc);
//     delay(&tim6, 1000);
//     hprintln!("wait !");


    

    

    
//     delay(&tim6, 1000);
//     hprintln!("hello world !");
//         loop {
            
            
//         }    

            
// }
// fn delay(tim6: &stm32f3::stm32f303::TIM6, ms: u16) {
//     // Set the timer to go off in `ms` ticks
//     // 1 tick = 1 ms
//     tim6.arr.write(|w| unsafe{
//         w.
//             arr().bits(ms)
//     });

//     // CEN: Enable the counter
//     tim6.cr1.modify(|_, w| w.cen().set_bit());

//     // Wait until the alarm goes off (until the update event occurs)
//     while !tim6.sr.read().uif().bit_is_set() {}

//     // Clear the update event flag
//     tim6.sr.modify(|_, w| w.uif().clear_bit());
// }

// fn delay_init(tim6: &stm32f3::stm32f303::TIM6, rcc: &stm32f3::stm32f303::RCC) {
//         //enabling tim6 clock
//     rcc.apb1enr.write(|w| 
//     w
//         .tim6en().set_bit()
//     );

//     //configuring the timer
//     tim6.cr1.write(|w|
//     w
//         .opm().set_bit() //one pulse mode selected
//         .cen().clear_bit() //disable counter
//     );

//     // Configure the prescaler to have the counter operate at 1 KHz
//     // APB1_CLOCK = 8 MHz
//     // PSC = 7999
//     // 8 MHz / (7999 + 1) = 1 KHz
//     // The counter (CNT) will increase on every millisecond
//     tim6.psc.write(|w| w.psc().bits(7999));
// }

//GPIO
#![no_std]
#![no_main]

extern crate panic_semihosting ;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use cortex_m_semihosting::hprint;


// use `main` as the entry point of this application
// `main` is not allowed to return

#[entry]
fn main() ->! {
    //initializing peripherals
    let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
    //calling rcc from library
    let rcc = peripherals.RCC;
    let gpioe = peripherals.GPIOE;

    //enabling input/output port
    rcc.ahbenr.write(|w|
        w 
            .iopeen().set_bit()
    );
    
    //mode selection
    gpioe.moder.write(|w|
    w
        .moder12().bits(0b01) //0b01 = output
    );

    //output type selection
    gpioe.otyper.write(|w|
        w
            .ot12().clear_bit() //pushpull
    );

    //output speed type selection
    gpioe.ospeedr.write(|w|
        w
            .ospeedr12().bits(0b01) //medium
    );

    //pullup/pulldown selection
    gpioe.pupdr.write(|w| unsafe { 
    w
        .pupdr12().bits(0b00) //no pullup pulldown
    });

    //set or reset the pin state
    gpioe.bsrr.write(|w|
    w
        .bs12().set_bit() //sets high
    );

    // gpioe.bsrr.write(|w|
    // w
    //     .bs12().set_bit() //sets low
    // );


    hprintln!("wait !");

        //io::init_porte(&rcc);

    hprintln!("hello world !");
        loop {
            
            
        }    

            
}

// struct io {
    
// }

// impl io {
  
//     fn init_porta (rcc: &stm32f3::stm32f303::RCC) {
//         //enabling input/output port
//         rcc.ahbenr.write(|w|
//         w 
//             .iopaen().set_bit()
//         );
//     }

//     fn init_portb (rcc: &stm32f3::stm32f303::RCC) {
//         //enabling input/output port
//         rcc.ahbenr.write(|w|
//         w 
//             .iopben().set_bit()
//         );
//     }

//     fn init_portc (rcc: &stm32f3::stm32f303::RCC) {
//         //enabling input/output port
//         rcc.ahbenr.write(|w|
//         w 
//             .iopcen().set_bit()
//         );
//     }

//     fn init_portd (rcc: &stm32f3::stm32f303::RCC) {
//         //enabling input/output port
//         rcc.ahbenr.write(|w|
//         w 
//             .iopden().set_bit()
//         );
//     }

//     fn init_porte (rcc: &stm32f3::stm32f303::RCC) {
//         //enabling input/output port
//         rcc.ahbenr.write(|w|
//         w 
//             .iopeen().set_bit()
//         );
//     }

//     fn init_portf (rcc: &stm32f3::stm32f303::RCC) {
//         //enabling input/output port
//         rcc.ahbenr.write(|w|
//         w 
//             .iopfen().set_bit()
//         );
//     }
// }