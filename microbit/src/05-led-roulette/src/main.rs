#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Board variables
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // This is hacky but quick lookup
    let idxs = [
        [0,0],
        [0,1],
        [0,2],
        [0,3],
        [0,4],
        [1,4],
        [2,4],
        [3,4],
        [4,4],
        [4,3],
        [4,2],
        [4,1],
        [4,0],
        [3,0],
        [2,0],
        [1,0],
    ];
    let mut idx : usize = 0;

    loop {
        let mut lights = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
        lights[idxs[idx][0]][idxs[idx][1]] = 1;
        display.show(&mut timer, lights, 25_u32);
        display.clear();

        idx += 1;
        idx = idx % idxs.len();
    }
}
