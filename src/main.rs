#![no_std]
#![no_main]


use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_time::Timer;
use static_cell::StaticCell;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use panic_probe as _;
use defmt_rtt as _;
use embassy_sync::channel::Channel;
use embassy_time::Duration;
use embassy_stm32::can::{CanTx, CanRx};
use embassy_stm32::can::Frame;
use crate::can_management::CanFrame;
use crate::can_management::messages::{Messages, ValveStatus, Pressure, Force};


mod can_management;


use can_management::can_controller::CanController;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut adc = Adc::new(p.ADC1);
    
    let mut brake_valve_1 = Output::new(p.PC6, Level::High, Speed::Low);
    let mut brake_valve_2 = Output::new(p.PC7, Level::High, Speed::Low);
   

    let mut can = CanController::new_can2(p.CAN2, p.PB12, p.PB13, 500_000, p.CAN1, p.PA11, p.PA12).await;
    let (mut can_tx,mut can_rx) = can.can.split();

    //spawner.spawn(can_writer(can_tx)).unwrap();
    //manda in can il primo messaggio di init

    Timer::after(Duration::from_secs(10)).await;
    let mut tank_1_sens = p.PA1;
    let mut tank_2_sens = p.PA2;
    let mut brake_front_sens = p.PA3; 
    let mut brake_rear_sens = p.PA4;

    let mut tank_1_press: f32;
    let mut tank_2_press: f32;
    let mut brake_front_press: f32; 
    let mut brake_rear_press: f32;

    let mut tank_1_force: f32;
    let mut tank_2_force: f32;
    let mut brake_front_force: f32;
    let mut brake_rear_force: f32;


    loop{
        for _i in 0..5 {
            tank_1_press = voltage_to_pressure_sp10(adc_to_voltage(adc.blocking_read(&mut tank_1_sens)));
            tank_2_press = voltage_to_pressure_sp10(adc_to_voltage(adc.blocking_read(&mut tank_2_sens)));
            brake_front_press = voltage_to_pressure_sp100(adc_to_voltage(adc.blocking_read(&mut brake_front_sens)));
            brake_rear_press = voltage_to_pressure_sp100(adc_to_voltage(adc.blocking_read(&mut brake_rear_sens)));

            tank_1_force      = pneumatic_press_to_force(tank_1_press);
            tank_2_force      = pneumatic_press_to_force(tank_2_press);
            brake_front_force = idraulic_press_to_force(brake_front_press);
            brake_rear_force  = idraulic_press_to_force(brake_rear_press);

            let pressure: Pressure = match Pressure::new(
                tank_1_press,
                tank_2_press,
                brake_front_press,
                brake_rear_press,
            ) {
            Ok(p) => p,
            Err(_) => {continue;}
            };

            let pressure_frame: Frame = match Frame::new_standard(
                Pressure::MESSAGE_ID as u16,
                pressure.raw(),
            ) {
                Ok(f) => f,
                Err(_) => {continue;}
            };
            can_tx.write(&pressure_frame).await;

            let force: Force = match Force::new(
                tank_1_force,
                tank_2_force,
                brake_front_force,
                brake_rear_force,
            ) {
            Ok(p) => p,
            Err(_) => {continue;}
            };

            let force_frame: Frame = match Frame::new_standard(
                Force::MESSAGE_ID as u16,
                force.raw(),
            ) {
                Ok(f) => f,
                Err(_) => {continue;}
            };
            can_tx.write(&force_frame).await;

            Timer::after(Duration::from_millis(500)).await;
        }

        

        //brake
        {
        brake_valve_1.set_low();
        brake_valve_2.set_low();

        let valve_status: ValveStatus = match ValveStatus::new(true, true) {
            Ok(p) => p,
            Err(_) => {continue;}
            };

            let valve_status_frame: Frame = match Frame::new_standard(
                ValveStatus::MESSAGE_ID as u16,
                valve_status.raw(),
            ) {
                Ok(f) => f,
                Err(_) => {continue;}
            };
            can_tx.write(&valve_status_frame).await;

        Timer::after(Duration::from_secs(5)).await;
        }

        for _i in 0..5 {
            tank_1_press = voltage_to_pressure_sp10(adc_to_voltage(adc.blocking_read(&mut tank_1_sens)));
            tank_2_press = voltage_to_pressure_sp10(adc_to_voltage(adc.blocking_read(&mut tank_2_sens)));
            brake_front_press = voltage_to_pressure_sp100(adc_to_voltage(adc.blocking_read(&mut brake_front_sens)));
            brake_rear_press = voltage_to_pressure_sp100(adc_to_voltage(adc.blocking_read(&mut brake_rear_sens)));

            tank_1_force      = pneumatic_press_to_force(tank_1_press);
            tank_2_force      = pneumatic_press_to_force(tank_2_press);
            brake_front_force = idraulic_press_to_force(brake_front_press);
            brake_rear_force  = idraulic_press_to_force(brake_rear_press);

            let pressure: Pressure = match Pressure::new(
                tank_1_press,
                tank_2_press,
                brake_front_press,
                brake_rear_press,
            ) {
            Ok(p) => p,
            Err(_) => {continue;}
            };

            let pressure_frame: Frame = match Frame::new_standard(
                Pressure::MESSAGE_ID as u16,
                pressure.raw(),
            ) {
                Ok(f) => f,
                Err(_) => {continue;}
            };
            can_tx.write(&pressure_frame).await;

            let force: Force = match Force::new(
                tank_1_force,
                tank_2_force,
                brake_front_force,
                brake_rear_force,
            ) {
            Ok(p) => p,
            Err(_) => {continue;}
            };

            let force_frame: Frame = match Frame::new_standard(
                Force::MESSAGE_ID as u16,
                force.raw(),
            ) {
                Ok(f) => f,
                Err(_) => {continue;}
            };
            can_tx.write(&force_frame).await;

            Timer::after(Duration::from_millis(500)).await;
        }
        //send value in can
        
        {
        //release brake
        brake_valve_1.set_high();
        brake_valve_2.set_high();

        let valve_status: ValveStatus = match ValveStatus::new(false, false) {
            Ok(p) => p,
            Err(_) => {continue;}
            };

            let valve_status_frame: Frame = match Frame::new_standard(
                ValveStatus::MESSAGE_ID as u16,
                valve_status.raw(),
            ) {
                Ok(f) => f,
                Err(_) => {continue;}
            };
            can_tx.write(&valve_status_frame).await;

        Timer::after(Duration::from_secs(5)).await;

        Timer::after(Duration::from_secs(5)).await;
        }
    }

}

/// Risoluzione dell'ADC in bit
pub const ADC_RESOLUTION: u32 = 12;
/// Valore massimo che può assumere l'ADC
pub const MAX_ADC_VALUE: f32 = (1 << ADC_RESOLUTION) as f32 - 1.0;
/// Tensione di riferimento dell'ADC
pub const ADC_VOLTAGE_REF: f32 = 3.3;

/// Range di tensione del sensore di pressione
pub const MIN_VOLTAGE: f32 = 0.5;
pub const MAX_VOLTAGE: f32 = 4.5;
/// Pressione massima misurabile (in Bar)
pub const MAX_PRESSURE_SP100: f32 = 100.0; //olio
pub const MAX_PRESSURE_SP10: f32 = 10.0;   //aria

/// Fattore di scala per convertire la tensione in pressione
pub const SCALING_FACTOR_SP100: f32 = MAX_PRESSURE_SP100 / (MAX_VOLTAGE - MIN_VOLTAGE);
pub const SCALING_FACTOR_SP10: f32 = MAX_PRESSURE_SP10 / (MAX_VOLTAGE - MIN_VOLTAGE);

/// Coefficienti di conversione da pressione a forza
pub const K_PP: f32 = 8.04e-4;
pub const K_PI: f32 = 1.54e-4;


/// Converte un valore ADC in tensione (V)
pub fn adc_to_voltage(adc_reading: u16) -> f32 {
    adc_reading as f32 * ADC_VOLTAGE_REF / MAX_ADC_VALUE
}

/// Converte una tensione in pressione (Bar)
pub fn voltage_to_pressure_sp100(voltage: f32) -> f32 {
    if voltage < MIN_VOLTAGE {
        0.0 // Se il valore è fuori scala, restituisce 0 Bar
    } else if voltage > MAX_VOLTAGE {
        MAX_PRESSURE_SP100 // Se è oltre il massimo, restituisce la pressione massima
    } else {
        SCALING_FACTOR_SP100 * (voltage - MIN_VOLTAGE)
    }
}

pub fn voltage_to_pressure_sp10(voltage: f32) -> f32 {
    if voltage < MIN_VOLTAGE {
        0.0 // Se il valore è fuori scala, restituisce 0 Bar
    } else if voltage > MAX_VOLTAGE {
        MAX_PRESSURE_SP10 // Se è oltre il massimo, restituisce la pressione massima
    } else {
        SCALING_FACTOR_SP10 * (voltage - MIN_VOLTAGE)
    }
}

pub fn pneumatic_press_to_force(pp: f32) -> f32 {
    pp * K_PP * 1e5
}

pub fn idraulic_press_to_force(pi: f32) -> f32 {
    pi * K_PI * 1e5
}


/*
#[embassy_executor::task]
async fn can_writer(mut tx: CanTx<'static>) {
    loop {
        let (id, mes) = CAN_WRITER.receive().await;
        let message = embassy_stm32::can::Frame::new_standard(id, &mes);
        if let Ok(some) = message {
            tx.write(&some).await;
        }
    }
}
*/