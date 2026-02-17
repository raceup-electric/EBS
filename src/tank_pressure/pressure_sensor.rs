use crate::TANK_STATUS;
use crate::TankPressure;
use crate::tank_pressure::filter_buffer::FilterBuffer;
use crate::tank_pressure::sensor::Sensor;
use crate::tank_pressure::utils::*;
use embassy_stm32::peripherals::{ADC1, ADC2, PA1, PA2};
use embassy_time::Timer;

pub const N_NEW_SAMPLES: usize = 10;

pub struct TankPressureSensor {
    sensor1: Sensor<ADC1, PA1>,
    sensor2: Sensor<ADC2, PA2>,
    buffer1: FilterBuffer,
    buffer2: FilterBuffer,
}

impl TankPressureSensor {
    pub fn new(sensor1: Sensor<ADC1, PA1>, sensor2: Sensor<ADC2, PA2>) -> Self {
        Self {
            sensor1,
            sensor2,
            buffer1: FilterBuffer::new(),
            buffer2: FilterBuffer::new(),
        }
    }

    pub fn measure(&mut self) {
        self.buffer1.add(self.sensor1.read());
        self.buffer2.add(self.sensor2.read());
    }

    pub fn get_pressure_one(&self) -> f32 {
        let voltage1 = adc_to_voltage(self.buffer1.avg());
        voltage_to_pressure(voltage1)
    }

    pub fn get_pressure_two(&self) -> f32 {
        let voltage2 = adc_to_voltage(self.buffer2.avg());
        voltage_to_pressure(voltage2)
    }
}

#[embassy_executor::task]
pub async fn tank_pressure_monitor(sensor: &'static mut TankPressureSensor) {
    loop {
        for _ in 0..N_NEW_SAMPLES {
            sensor.measure();
            Timer::after_millis(10).await;
        }
        TANK_STATUS.signal(TankPressure::new(
            sensor.get_pressure_one(),
            sensor.get_pressure_two(),
        ));
        Timer::after_millis(1).await;
    }
}

