#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

// This is hacky but quick lookup
const IDXS : [(usize, usize); 16] = [
    ( 0,0 ),
    ( 0,1 ),
    ( 0,2 ),
    ( 0,3 ),
    ( 0,4 ),
    ( 1,4 ),
    ( 2,4 ),
    ( 3,4 ),
    ( 4,4 ),
    ( 4,3 ),
    ( 4,2 ),
    ( 4,1 ),
    ( 4,0 ),
    ( 3,0 ),
    ( 2,0 ),
    ( 1,0 ),
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Board variables
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);


    let mut lights = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut last_light = (0,0);

    loop {
        for light in IDXS.iter() {
            lights[last_light.0][last_light.1] = 0;
            lights[light.0][light.1] = 1;
            display.show(&mut timer, lights, 25_u32);
            last_light = *light;
        }
    }
}
