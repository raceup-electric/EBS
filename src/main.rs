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
use crate::can_management::messages::{Voltage, Pressure, Force, Raw};

mod can_management;

use can_management::can_controller::CanController;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut adc = Adc::new(p.ADC1);
    
    let mut brake_valve_1 = Output::new(p.PC6, Level::High, Speed::Low);
    let mut brake_valve_2 = Output::new(p.PC7, Level::High, Speed::Low);
   

    let mut can = CanController::new_can2(
        p.CAN2, p.PB12, p.PB13, 500_000,
        p.CAN1, p.PA11, p.PA12,
    ).await;
    let (mut can_tx, mut _can_rx) = can.can.split();

    Timer::after(Duration::from_secs(10)).await;

    let mut tank_sense    = p.PA2;
    let mut brake_sense   = p.PA1;

    let mut tank_raw:      u16;
    let mut brake_raw:    u16;
    
    let mut brake_volt:   f32;
    let mut tank_volt:     f32;
    
    let mut brake_press:  f32;
    let mut tank_press:    f32;

    let mut tank_force:    f32;
    let mut brake_force:   f32;

    loop {
        
        tank_raw   = adc.blocking_read(&mut tank_sense);
        tank_volt  = adc_to_voltage(tank_raw) * 2.0; //il voltaggio viene dimezzato prima di essere letto
        tank_press = voltage_to_pressure_sp10(tank_volt);
        tank_force = pneumatic_press_to_force(tank_press);

        brake_raw   = adc.blocking_read(&mut brake_sense);
        brake_volt  = adc_to_voltage(brake_raw) * 2.0; //il voltaggio viene dimezzato prima di essere letto
        brake_press = voltage_to_pressure_sp100(brake_volt);
        brake_force = hydraulic_press_to_force(brake_press);

        // PRESSURE
        match Pressure::new(tank_press, brake_press) {
            Ok(pressure_msg) => {
                match Frame::new_standard(
                    Pressure::MESSAGE_ID as u16,
                    pressure_msg.raw(),
                ) {
                    Ok(pressure_frame) => {
                        can_tx.write(&pressure_frame).await;
                    }
                    Err(_) => {
                        if let Ok(err_frame) = Frame::new_standard(1u16, &[]) {
                            can_tx.write(&err_frame).await;
                        }
                    }
                }
            }
            Err(_) => {
                let err_payload = [0xFFu8; 8];
                if let Ok(err_frame) = Frame::new_standard(
                    Pressure::MESSAGE_ID as u16,
                    &err_payload,
                ) {
                    can_tx.write(&err_frame).await;
                } else if let Ok(fail_frame) = Frame::new_standard(1u16, &[]) {
                    can_tx.write(&fail_frame).await;
                }
            }
        }

        // FORCE
        match Force::new(tank_force, brake_force) {
            Ok(force_msg) => {
                match Frame::new_standard(
                    Force::MESSAGE_ID as u16,
                    force_msg.raw(),
                ) {
                    Ok(force_frame) => {
                        can_tx.write(&force_frame).await;
                    }
                    Err(_) => {
                        if let Ok(err_frame) = Frame::new_standard(1u16, &[]) {
                            can_tx.write(&err_frame).await;
                        }
                    }
                }
            }
            Err(_) => {
                let err_payload = [0xFFu8; 8];
                if let Ok(err_frame) = Frame::new_standard(
                    Force::MESSAGE_ID as u16,
                    &err_payload,
                ) {
                    can_tx.write(&err_frame).await;
                } else if let Ok(fail_frame) = Frame::new_standard(1u16, &[]) {
                    can_tx.write(&fail_frame).await;
                }
            }
        }

        //  VOLTAGE
        match Voltage::new(tank_volt, brake_volt) {
            Ok(volt_msg) => {
                match Frame::new_standard(
                    Voltage::MESSAGE_ID as u16,
                    volt_msg.raw(),
                ) {
                    Ok(volt_frame) => {
                        can_tx.write(&volt_frame).await;
                    }
                    Err(_) => {
                        if let Ok(err_frame) = Frame::new_standard(1u16, &[]) {
                            can_tx.write(&err_frame).await;
                        }
                    }
                }
            }
            Err(_) => {
                let err_payload = [0xFFu8; 8];
                if let Ok(err_frame) = Frame::new_standard(
                    Voltage::MESSAGE_ID as u16,
                    &err_payload,
                ) {
                    can_tx.write(&err_frame).await;
                } else if let Ok(fail_frame) = Frame::new_standard(1u16, &[]) {
                    can_tx.write(&fail_frame).await;
                }
            }
        }

        //  RAW
        match Raw::new(tank_raw, brake_raw) {
            Ok(raw_msg) => {
                match Frame::new_standard(
                    Raw::MESSAGE_ID as u16,
                    raw_msg.raw(),
                ) {
                    Ok(pressure_frame) => {
                        can_tx.write(&pressure_frame).await;
                    }
                    Err(_) => {
                        if let Ok(err_frame) = Frame::new_standard(1u16, &[]) {
                            can_tx.write(&err_frame).await;
                        }
                    }
                }
            }
            Err(_) => {
                let err_payload = [0xFFu8; 8];
                if let Ok(err_frame) = Frame::new_standard(
                    Raw::MESSAGE_ID as u16,
                    &err_payload,
                ) {
                    can_tx.write(&err_frame).await;
                } else if let Ok(fail_frame) = Frame::new_standard(1u16, &[]) {
                    can_tx.write(&fail_frame).await;
                }
            }
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
        0.0
    } else if voltage > MAX_VOLTAGE {
        MAX_PRESSURE_SP100
    } else {
        SCALING_FACTOR_SP100 * (voltage - MIN_VOLTAGE)
    }
}

pub fn voltage_to_pressure_sp10(voltage: f32) -> f32 {
    if voltage < MIN_VOLTAGE {
        0.0
    } else if voltage > MAX_VOLTAGE {
        MAX_PRESSURE_SP10
    } else {
        SCALING_FACTOR_SP10 * (voltage - MIN_VOLTAGE)
    }
}

pub fn pneumatic_press_to_force(pp: f32) -> f32 {
    pp * K_PP * 1e5
}

pub fn hydraulic_press_to_force(pi: f32) -> f32 {
    pi * K_PI * 1e5
}

