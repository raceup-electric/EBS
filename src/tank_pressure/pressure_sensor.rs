use embassy_time::Timer;
use crate::tank_pressure::sensor::Sensor;
use crate::tank_pressure::filter_buffer::FilterBuffer;
use crate::tank_pressure::utils::*;
use embassy_stm32::peripherals::{ADC1, ADC2, PA1, PA2};
use embassy_stm32::Peri;
use crate::TANK_STATUS;
use crate::TankStatus;  





pub const N_NEW_SAMPLES: usize = 10;

pub struct TankPressureSensor {
    sensor1: Sensor<ADC1, Peri<'static, PA1>>,
    sensor2: Sensor<ADC2, Peri<'static, PA2>>,
    buffer1: FilterBuffer,
    buffer2: FilterBuffer,
}

impl TankPressureSensor {
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
pub async fn tank_pressure_monitor(sensor: &'static mut TankPressureSensor) {
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

        let sensor1_check = true; //TODO, check per i sensori in base ai valori nel
        let sensor2_check = true;

        TANK_STATUS.signal(
            TankStatus::new(
                pressure1,
                pressure2,
                sensor1_check,
                sensor2_check,
            )
        );
        Timer::after_millis(200).await;
    }
}