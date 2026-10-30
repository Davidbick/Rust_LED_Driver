#![no_std]
#![no_main]

mod animations;
mod lis3dh;
mod neopixel_matrix;

use cortex_m_rt::entry;
use panic_halt as _;
use hal::pio::PIOExt;
use embedded_hal::digital::v2::OutputPin;
use adafruit_feather_rp2040::hal;
use adafruit_feather_rp2040::{Pins, XOSC_CRYSTAL_FREQ};
use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio,
};
use cortex_m::delay::Delay;
use smart_leds::SmartLedsWrite;
use ws2812_pio::Ws2812;
use animations::{
    Animation,
    HeartPulse,
    ZanScroll,
    StarryTwinkle,
    TrinidadFlag,
};
use lis3dh::Lis3dh;
use neopixel_matrix::NeoMatrix;
use fugit::RateExtU32;

const THRESHOLD: f32 = 0.25;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    
    let sda_pin = pins.sda.into_function::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.scl.into_function::<hal::gpio::FunctionI2C>();
    
    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.system_clock.freq(),
    );

    let mut accel = Lis3dh::new(i2c);
    accel.init().ok();

    let power_pin = pins.d10.into_push_pull_output();
    let mut matrix = NeoMatrix::new(power_pin);
    matrix.init();

    let neopixel_pin = pins.d5.into_function::<hal::gpio::FunctionPio0>();    
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        neopixel_pin,
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut pos_x_anim = HeartPulse::new();
    let mut neg_x_anim = ZanScroll::new();
    let mut pos_y_anim = StarryTwinkle::new();
    let mut neg_y_anim = TrinidadFlag::new();

    loop {
        accel.update().ok();

        let x = accel.x;
        let y = accel.y;

        if x.abs() >= y.abs() {
            if x >= THRESHOLD {
                pos_x_anim.next();
                let frame = pos_x_anim.to_list();
                matrix.load_frame(&frame);
            } else if x <= -THRESHOLD {
                neg_x_anim.next();
                let frame = neg_x_anim.to_list();
                matrix.load_frame(&frame);
            } else {
                matrix.clear_pixels();
            }
        } else {
            if y >= THRESHOLD {
                pos_y_anim.next();
                let frame = pos_y_anim.to_list();
                matrix.load_frame(&frame);
            } else if y <= -THRESHOLD {
                neg_y_anim.next();
                let frame = neg_y_anim.to_list();
                matrix.load_frame(&frame);
            } else {
                matrix.clear_pixels();
            }
        }

        let _ = matrix.write(&mut ws);
        delay.delay_ms(120);
    }
}
