/*
pub mod pressure_thresholds {
    pub const FRONT_PRESS_MULT :f32 = 3.5 ;
    pub const REAR_PRESS_MULT  :f32 = 3.0 ;
    pub const MIN_FRONT_PRESS  :f32 = 30.0 ;
    pub const MIN_REAR_PRESS   :f32 = 22.0 ;
    pub const MIN_TANK_PRESS   :f32 = 6.0;
    pub const MAX_TANK_PRESS   :f32 = 10.0;
}
*/
pub mod pressure_thresholds {
    pub const FRONT_PRESS_MULT: f32 = 1.5;
    pub const REAR_PRESS_MULT: f32 = 1.5;
    pub const MIN_FRONT_PRESS: f32 = 10.0;
    pub const MIN_REAR_PRESS: f32 = 10.0;
    pub const MIN_TANK_PRESS: f32 = 6.0;
    pub const MAX_TANK_PRESS: f32 = 10.0;
}

/*
pub mod peripherals_type { //da mettere nel main
    use embassy_stm32::peripherals::*;
    type AdcPinTank1 = ADC1;
    type AdcPinTank2 = ADC2;
    type PinTank1 = PA1;
    type PinTank2 = PA2;
}
*/

