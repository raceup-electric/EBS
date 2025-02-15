use embassy_sync::{mutex::Mutex};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::Timer;
use defmt::info;
use crate::can_management::can_operation;
use crate::pressure::sensor::Sensor;
use crate::pressure::filter_buffer::FilterBuffer;
use crate::pressure::utils::*;
use embassy_stm32::peripherals::{ADC1, ADC2, PA1, PA2};
use super::super::can_management;
use can_management::can_controller::CanController;





pub const N_NEW_SAMPLES: usize = 5;

pub struct BrakePressureSensor {
    sensor1: Sensor<ADC1, PA1>,
    sensor2: Sensor<ADC2, PA2>,
    buffer1: FilterBuffer,
    buffer2: FilterBuffer,
}

impl BrakePressureSensor {
    pub fn new(sensor1: Sensor<ADC1, PA1>, sensor2: Sensor<ADC2, PA2>) -> Self {
        Self {
            sensor1,
            sensor2,
            buffer1: FilterBuffer::new(),
            buffer2: FilterBuffer::new(),
        }
    }

}

#[embassy_executor::task]
pub async fn brake_pressure_monitor(sensor: &'static mut BrakePressureSensor, can: &'static Mutex<CriticalSectionRawMutex, CanController<'static>>) {
    loop {
        for _ in 0..N_NEW_SAMPLES {
            let val1 = sensor.sensor1.read();
            let val2 = sensor.sensor2.read();
            sensor.buffer1.add(val1);
            sensor.buffer2.add(val2);

            Timer::after_millis(10).await; //TODO, il tempo di attesa qui deve dipendere dal numero new sample e dalla frequenza di invio delle pressioni
        }

        let voltage1 = adc_to_voltage(sensor.buffer1.avg());
        let voltage2 = adc_to_voltage(sensor.buffer2.avg());

        info!("Left voltage = {}, Right voltage = {}", voltage1, voltage2);

        let pressure1 = voltage_to_pressure(voltage1);
        let pressure2 = voltage_to_pressure(voltage2);

        info!("Left Brake pressure = {}, Right Brake pressure = {}", pressure1, pressure2);

        let mut can_data = can.lock().await;
        can_operation(&mut can_data, voltage1 as u16, voltage2 as u16);


        //parsing
        //invia nel channel can il frame
        Timer::after_millis(200).await;

    }
}