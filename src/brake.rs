use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_stm32::peripherals::{PC6, PC7};
pub enum BrakeSignal {
    Engage,
    Release,
    TankOneCheck,
    TankTwoCheck
}

#[derive(defmt::Format)]
#[derive(PartialEq)]
pub enum BrakeStatus {
    Engaged,
    Released
}

pub struct BrakeController {
    pin1: Output<'static>,
    pin2: Output<'static>,
    status: BrakeStatus,
}

impl BrakeController {
    pub fn new(pin1: PC6, pin2: PC7) -> Self {
        Self {
            pin1: Output::new(pin1, Level::High, Speed::Low),
            pin2: Output::new(pin2, Level::High, Speed::Low),
            status: BrakeStatus::Released,
        }
    }

    pub fn release(&mut self) {
        self.pin1.set_high();
        self.pin2.set_high();
        self.status = BrakeStatus::Released;
    }

    pub fn handle_signal(&mut self, signal: BrakeSignal) {
        match signal {
            BrakeSignal::Release =>  {
                self.release();
            }
                
            _ => {
                self.pin1.set_low();
                self.pin2.set_low();
                self.status = BrakeStatus::Engaged;
            }
        }
    }
}
