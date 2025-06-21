#![no_std]
#![no_main]

use embassy_executor::Spawner;
use defmt_rtt as _;
use panic_probe as _;
use embassy_stm32::gpio::{Output, Level, Speed};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    Output::new(p.PC6, Level::High, Speed::Low).set_high();
    Output::new(p.PC7, Level::High, Speed::Low).set_high();
}
