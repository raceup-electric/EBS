use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_stm32::peripherals::{PC6, PC7};

pub enum BrakeSignal {
    Engage,
    Release,
    CheckStatus,
}

#[derive(defmt::Format)]
pub enum BrakeStatus {
    Engaged,
    Released,
    NotSet,
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
            status: BrakeStatus::NotSet,
        }
    }

    pub fn engage(&mut self) {
        self.pin1.set_low();
        self.pin2.set_low();
        self.status = BrakeStatus::Engaged;
        defmt::info!("Freno ENGAGED");
    }

    pub fn release(&mut self) {
        self.pin1.set_high();
        self.pin2.set_high();
        self.status = BrakeStatus::Released;
        defmt::info!("Freno RELEASED");
    }

    pub fn check_status(&self) -> &BrakeStatus {
        defmt::info!("Stato attuale del freno: {:?}", &self.status);
        &self.status
    }

    pub fn set_status(&mut self, status: BrakeStatus) {
        self.status = status;
    }
}
