#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal as hal;
use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use hal::adc::SampleTime;
use hal::gpio::{Level, Output};
use hal::println;
use qingke::riscv::asm::nop;

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(spawner: Spawner) -> ! {

    let mut config = hal::Config::default();
    let p = hal::init(config);

    let mut delay = Delay;

    let mut adcU = hal::adc::Adc::new(p.ADC1, Default::default());
    let mut adcW = hal::adc::Adc::new(p.ADC2,Default::default());

    let mut pIU = p.PA3;
    let mut pIW = p.PA4;

    // GPIO
    let mut led = Output::new(p.PB1, Level::Low, Default::default());

    loop {
        led.set_high();
        //Timer::after(Duration::from_millis(500)).await;
        for _ in 0..14400000 {
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();

        }
        led.set_low();
        for _ in 0..14400000 {
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();
            nop();

        }
        //Timer::after(Duration::from_millis(500)).await;

        //let val = adcU.convert(&mut pIU, SampleTime::CYCLES239_5);

    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = println!("\n\n\n{}", info);

    loop {}
}
