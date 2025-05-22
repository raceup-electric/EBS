use embassy_time::Timer;
use crate::tank_pressure::sensor::Sensor;
use crate::tank_pressure::filter_buffer::FilterBuffer;
use crate::tank_pressure::utils::*;
use embassy_stm32::peripherals::{ADC1, ADC2, PA1, PA2};
use super::super::can_management;
use can_management::can_controller::CanController;
use can_management::messages::TankEbs;
use embassy_stm32::Peri;





pub const N_NEW_SAMPLES: usize = 5;

pub struct BrakePressureSensor {
    sensor1: Sensor<ADC1, Peri<'static, PA1>>,
    sensor2: Sensor<ADC2, Peri<'static, PA2>>,
    buffer1: FilterBuffer,
    buffer2: FilterBuffer,
}

impl BrakePressureSensor {
    pub fn new(sensor1: Sensor<ADC1, Peri<'static, PA1>>, sensor2: Sensor<ADC2, Peri<'static,  PA2>>) -> Self {
        Self {
            sensor1,
            sensor2,
            buffer1: FilterBuffer::new(),
            buffer2: FilterBuffer::new(),
        }
    }

}

#[embassy_executor::task]
pub async fn brake_pressure_monitor(sensor: &'static mut BrakePressureSensor) {
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

        let pressure1 = voltage_to_pressure(voltage1);
        let pressure2 = voltage_to_pressure(voltage2);

        let pressure_msg = pressure_to_can_msg((pressure1, pressure2));
        //let mut can_data = can.lock().await;
        //can_operation(&mut can_data, sensor.buffer1.avg() as u16, sensor.buffer2.avg() as u16).await;


        //parsing
        //invia nel channel can il frame
        Timer::after_millis(200).await;

    }
}

fn pressure_to_can_msg(pressure: (f32, f32)) -> TankEbs{
    TankEbs::new(brake_pressure_is_critical(pressure), pressure.1, pressure.2, true, true)
}