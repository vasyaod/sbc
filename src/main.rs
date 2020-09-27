#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate cortex_m as cm;
//use cm::{interrupt::Mutex};
//use cm::peripheral::Peripherals;
//use cortex_m_rt::entry;

extern crate cortex_m_semihosting;
//use cortex_m_semihosting::hprintln;

extern crate panic_semihosting;

//extern crate stm32f0;
//use stm32f0::stm32f0x0;

extern crate stm32f0xx_hal as hal;
use hal::prelude::*;
use hal::stm32;
use hal::stm32::{interrupt, Interrupt, Peripherals, TIM3, EXTI, I2C1};
use hal::gpio::*;
use hal::{
    i2c::I2c,//i2c::{BlockingI2c, DutyCycle, Mode},
    time::Hertz,
    timers::*,
};

use rtfm::app;

mod interface;
use interface::i2c::I2cInterface;

mod ssd13xx;

#[app(device = hal::stm32)]
const APP: () = {

    struct Resources {
        led: gpiob::PB1<Output<PushPull>>,
        timer: Timer<TIM3>,
        exti: EXTI,
        display: ssd13xx::SSD13xx<interface::i2c::I2cInterface<hal::i2c::I2c<hal::stm32::I2C1, hal::gpio::gpioa::PA9<hal::gpio::Alternate<hal::gpio::AF4>>, hal::gpio::gpioa::PA10<hal::gpio::Alternate<hal::gpio::AF4>>>>>,
        counter: u32,
     //   i2c: I2c<I2C1, gpioa::PA9<Alternate<AF4>>, gpioa::PA10<Alternate<AF4>>>
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources{
        
        let res: init::LateResources = cortex_m::interrupt::free(|cs| {
            let mut peripherals = stm32::Peripherals::take().unwrap();
            let cp = cm::peripheral::Peripherals::take().unwrap();
            
           // cortex_m::interrupt::enable();
            // Enable clock for SYSCFG
            let rcc = peripherals.RCC;
            rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    
          //  let rcc = &peripherals.RCC;
            let mut rcc = rcc
                .configure()
                .sysclk(8.mhz())
                .freeze(&mut peripherals.FLASH);

    
            let gpiob = peripherals.GPIOB.split(&mut rcc);
            let gpioa = peripherals.GPIOA.split(&mut rcc);
          //  let syst = peripherals.SYST.split(&mut rcc);
            let syscfg = peripherals.SYSCFG;
            let exti = peripherals.EXTI;

           // let _ = cortex_m::interrupt::free(|cs| gpioa.pa0.into_pull_down_input(cs));
            // let _ = cortex_m::interrupt::free(|cs| gpioa.pa0.into_pull_down_input(cs));
            // let mut led = cortex_m::interrupt::free(|cs| gpiob.pb1.into_push_pull_output(cs));
            //let mut tx = cortex_m::interrupt::free(|cs| gpioa.pa9.into_alternate_af1(cs));
            let led = gpiob.pb1.into_push_pull_output(cs);

            gpioa.pa1.into_pull_down_input(cs);
            
            // Enable external interrupt for PA1
            syscfg.exticr1.modify(|_, w| unsafe { w.exti0().bits(0) });
        
            // Set interrupt request mask for line 1
            exti.imr.modify(|_, w| w.mr0().set_bit());
        
            // Set interrupt rising trigger for line 1
            exti.rtsr.modify(|_, w| w.tr0().set_bit());

            let mut timer = Timer::tim3(peripherals.TIM3, Hertz(1), &mut rcc);
            // // // Generate an interrupt when the timer expires
            timer.listen(Event::TimeOut);
    
            let sda: gpioa::PA10<Alternate<AF4>> = gpioa.pa10.into_alternate_af4(cs);
            let scl: gpioa::PA9<Alternate<AF4>> = gpioa.pa9.into_alternate_af4(cs);

            
            let i2c = I2c::i2c1(peripherals.I2C1, (scl, sda), 400.khz(), &mut rcc);
            let i2c_add = 0x3C;

            let mut display = ssd13xx::SSD13xx::new(I2cInterface::new(i2c, i2c_add));

            display.init().ok();
            display.clear().ok();
            //display.draw_char(&16, &16, &'6').ok();
          //  display.draw_text(&0, &0, &"Hello World!!!").ok();
            display.draw_int(&0, &0, &123456).ok();
            
            init::LateResources { 
                led: led,  
                timer: timer,
                exti: exti,
                display: display,
                counter: 100
            //    display: display
           //     i2c: i2c
            }
        });
        res
    }

    // #[idle]
    // fn idle(_: idle::Context) -> ! {
    //     loop {
    //         continue;
    //     }
    // }

    // #[task(resources = [led])]
    // fn foo(ctx: foo::Context) {
    //     ctx.resources.led.toggle().ok();
    // }

    // extern "C" {
    //     fn USART1();
    //     fn USART2();
    // }

    #[task(binds = EXTI0_1, resources = [led, timer, exti, display, counter])]
    fn ext01(cxt: ext01::Context) {
        cortex_m::interrupt::free(|cs| {
          //  cxt.resources.led.set_low().ok();
            *cxt.resources.counter = *cxt.resources.counter + 1;
          //  cxt.resources.display.draw_text(&0, &0, &"           ").ok();
            cxt.resources.display.draw_int(&0, &0, &((*cxt.resources.counter) as usize)).ok();
            //cxt.resources.exti.pr.modify(|_, w| w.pif1().set_bit());
            cxt.resources.exti.pr.write(|w| w.pr0().set_bit());
        })
    }

    #[task(binds = TIM3, resources = [led, timer])]
    fn tim3(cxt: tim3::Context) {
        cortex_m::interrupt::free(|cs| {
            cxt.resources.led.toggle().ok();
            cxt.resources.timer.wait().ok();
        })
    }
};
