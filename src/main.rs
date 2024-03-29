#![no_std]
#![no_main]

#[cfg(feature = "panic_halt")]
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::{board::Board, display::blocking::Display, hal::Timer};

#[entry]
fn entry() -> ! {
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // display lattice (matrix)
    let mut disp_latt = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    // 5 extra spaces at end servers for fade-out effect
    const TEXT: &str = "UGLY-MAXIMAL abcdefghijklmnopqrstuvwxyz 1234567890";
    const extra_sp: usize = 5;
    let mut col_defs = [<&[u8]>::default(); ug_max::buff_size(TEXT, extra_sp)];
    ug_max::col_defs(TEXT, extra_sp, &mut col_defs);    

    // for ever
    loop {
        // traverse translation
        for defintion in col_defs {
            // over columns of unit
            for column in defintion {
                // shift previous screen left
                for cix in 1..5 {
                    let prev_cix = cix - 1;
                    for rix in 0..5 {
                        disp_latt[rix][prev_cix] = disp_latt[rix][cix];
                    }
                }

                // append new column
                for rix in 0..5 {
                    let mask = 1 << rix;
                    disp_latt[rix][4] = if column & mask == mask { 1 } else { 0 };
                }

                display.show(&mut timer, disp_latt, 150);
            }
        }
    }
}

#[cfg(feature = "panic_abort")]
mod panic_abort {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

// cargo flash  --target thumbv7em-none-eabihf --chip nRF52833_xxAA --release --features panic_abort
// cargo flash  --target thumbv7em-none-eabihf --chip nRF52833_xxAA --features panic_halt
// cargo build --release  --target thumbv7em-none-eabihf --features panic_abort
// cargo build --target thumbv7em-none-eabihf --features panic_halt
