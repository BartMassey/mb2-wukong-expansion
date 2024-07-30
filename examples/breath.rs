#![no_main]
#![no_std]

use panic_probe as _;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        self,
        twim,
        pac::twim0,
    },
};

use mb2_wukong_expansion::{WuKong, LightMode};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_internal.into(),
        twim0::frequency::FREQUENCY_A::K100,
    );
    let timer = hal::Timer::new(board.TIMER0);
    let mut wukong = WuKong::new(timer, i2c);
    wukong.set_light_mode(LightMode::Breath).unwrap();

    loop {
        wfi();
    }
}
