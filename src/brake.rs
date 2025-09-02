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

pub enum Tank {
    One,
    Two
}
pub struct BrakeController {
    pin1: Output<'static>,
    pin2: Output<'static>,
    last_tank_utilized: Tank,
    status: BrakeStatus,
}

impl BrakeController {
    pub fn new(pin1: PC6, pin2: PC7) -> Self {
        Self {
            pin1: Output::new(pin1, Level::High, Speed::Low),
            pin2: Output::new(pin2, Level::High, Speed::Low),
            last_tank_utilized : Tank::One,
            status: BrakeStatus::Released,
        }
    }

    pub fn engage(&mut self) {
        if self.status == BrakeStatus::Released {
            match self.last_tank_utilized {
                Tank::One => { //now brake realising second tank
                    self.pin1.set_high();
                    self.pin2.set_low();
                    self.last_tank_utilized = Tank::Two;
                }
                Tank::Two => { //now brake realising first tank
                    self.pin1.set_low();
                    self.pin2.set_high();
                    self.last_tank_utilized = Tank::One;
                }    
            }
            self.status = BrakeStatus::Engaged;
        }
    }

    pub fn release(&mut self) {
        self.pin1.set_high();
        self.pin2.set_high();
        self.status = BrakeStatus::Released;
    }

    pub fn handle_signal(&mut self, signal: BrakeSignal) {
        match signal {
            BrakeSignal::Engage =>  {
                self.engage();
                self.status = BrakeStatus::Engaged;
            }
                
            BrakeSignal::Release =>  {
                self.release();
                self.status = BrakeStatus::Released;
            }

            BrakeSignal::TankOneCheck => {
                self.pin1.set_low();
                self.pin2.set_high();
                self.last_tank_utilized = Tank::One;
                self.status = BrakeStatus::Engaged;
            }

            BrakeSignal::TankTwoCheck => {
                self.pin1.set_high();
                self.pin2.set_low();
                self.last_tank_utilized = Tank::Two;
            }
        }
    }
}
