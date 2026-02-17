use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals::{PC6, PC7};
#[allow(dead_code)]
pub enum BrakeSignal {
    Engage,
    Release,
    TankOneCheck,
    TankTwoCheck,
    DoubleBrake,
}

#[derive(defmt::Format, PartialEq)]
pub enum BrakeStatus {
    Engaged,
    Released,
}

#[allow(dead_code)]
pub enum Tank {
    One,
    Two,
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

    pub fn handle_signal(&mut self, signal: BrakeSignal, press_tank_one: f32, press_tank_two: f32) {
        match signal {
            BrakeSignal::Engage => {
                if press_tank_one > press_tank_two {
                    self.pin1.set_low();
                    self.pin2.set_high();
                } else {
                    self.pin1.set_high();
                    self.pin2.set_low();
                }
                self.status = BrakeStatus::Engaged;
            }

            BrakeSignal::Release => {
                self.pin1.set_high();
                self.pin2.set_high();
                self.status = BrakeStatus::Released;
            }

            BrakeSignal::TankOneCheck => {
                self.pin1.set_low();
                self.pin2.set_high();
                self.status = BrakeStatus::Engaged;
            }

            BrakeSignal::TankTwoCheck => {
                self.pin1.set_high();
                self.pin2.set_low();
                self.status = BrakeStatus::Engaged;
            }

            BrakeSignal::DoubleBrake => {
                self.pin1.set_low();
                self.pin2.set_low();
                self.status = BrakeStatus::Engaged;
            }
        }
    }
}
