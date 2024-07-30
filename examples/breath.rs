#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use microbit::{board::Board, hal};

use mb2_wukong_expansion::{WuKong, LightMode};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let delay = hal::Timer::new(board.TIMER0);
    let i2c = board.i2c_external;
    let mut wukong = WuKong::new(delay, board.TWIM0, i2c.scl, i2c.sda);
    wukong.set_light_mode(LightMode::Breath).unwrap();

    loop {
        wfi();
    }
}
