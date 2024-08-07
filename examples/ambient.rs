#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    board::Board,
    hal::{
        gpio::{DriveConfig, Level},
        Timer,
    },
};

use mb2_wukong_expansion::{WuKongAmbient, RGB8};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let wka_delay = Timer::new(board.TIMER0);
    let mut delay = Timer::new(board.TIMER1);
    let pin = board
        .edge
        .e16
        .into_push_pull_output_drive(Level::Low, DriveConfig::HighDrive0HighDrive1);
    let mut wka = WuKongAmbient::new(board.PWM0, wka_delay, pin).unwrap();
    let mut red = 0u8;
    loop {
        let rgb = RGB8::new(red, 64, 64);
        wka.set_color(3, rgb).unwrap();
        delay.delay_ms(20);
        red = red.wrapping_add(1);
    }
}
