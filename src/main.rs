#![no_std]
#![no_main]

use brake::BrakeController;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::peripherals::{ADC1, ADC2, PA1, PA2};
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_time::Timer;
use defmt::info;
use static_cell::StaticCell;
use embassy_sync::signal::Signal;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

mod pressure;
mod brake;
mod can_management;

use can_management::can_controller::CanController;

use pressure::sensor::Sensor;
use pressure::pressure_sensor::{BrakePressureSensor, brake_pressure_monitor};
use brake::{BrakeSignal, BrakeStatus};

static BRAKE_PRESSURE_SENSOR: StaticCell<BrakePressureSensor> = StaticCell::new();

static BRAKE_CONTROLLER: StaticCell<BrakeController> = StaticCell::new();

static BRAKE_SIGNAL: Signal<CriticalSectionRawMutex, brake::BrakeSignal> = Signal::new();

static CAN: StaticCell<Mutex<CriticalSectionRawMutex, CanController>> = StaticCell::new();

static PRESSURE_READING: StaticCell<Mutex<CriticalSectionRawMutex, (f32, f32)>> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let brake_pressure_sensor = BRAKE_PRESSURE_SENSOR.init(BrakePressureSensor::new(
        Sensor::new(Adc::new(p.ADC1), p.PA1),
        Sensor::new(Adc::new(p.ADC2), p.PA2),
    ));
    let can = CanController::new_can2(p.CAN2, p.PB12, p.PB13, 500_000, p.CAN1, p.PA11, p.PA12).await;
    let can_mutex = Mutex::new(can);
    let can = StaticCell::init(&CAN, can_mutex);
    spawner.spawn(send_can(can)).unwrap();

    spawner.spawn(brake_pressure_monitor(brake_pressure_sensor, can)).unwrap();
     
    let brake_controller = BRAKE_CONTROLLER.init(BrakeController::new(p.PC6,p.PC7));

    spawner.spawn(brake_control_task(brake_controller));



    /*
    let mut brake_left = Output::new(p.PC6, Level::High, Speed::Low);
    let mut brake_right = Output::new(p.PC7, Level::High, Speed::Low);


    loop {
        brake_left.set_low();
        brake_right.set_low();
        info!("Brake ON");
        Timer::after_millis(1000).await;

        brake_left.set_high();
        brake_right.set_high();
        info!("Brake OFF");CriticalSectionMutex
        Timer::after_millis(1000).await;
    }*/ //deccommentare se non funziona quello sotto

    loop{
        engage_brake();
        Timer::after_millis(400).await;
        release_brake();
        Timer::after_millis(400).await;
    }

    


}

#[embassy_executor::task]
async fn brake_control_task(brake: &'static mut BrakeController) {
    loop {
        let command = BRAKE_SIGNAL.wait().await;
        
        match command {
            BrakeSignal::Engage => {
                brake.engage();
                brake.set_status(BrakeStatus::Engaged);
            }
            BrakeSignal::Release => {
                brake.release();
                brake.set_status(BrakeStatus::Released);
            }
            BrakeSignal::CheckStatus => {
                let status = brake.check_status();
                info!("Brake status: {:?}", status);
            }
        }
    }
}

pub fn engage_brake() {
    BRAKE_SIGNAL.signal(BrakeSignal::Engage);
}

pub fn release_brake() {
    BRAKE_SIGNAL.signal(BrakeSignal::Release);
}

#[embassy_executor::task]
async fn send_can(
    can: &'static Mutex<CriticalSectionRawMutex, CanController<'static>>
){
    loop {
        embassy_time::Timer::after_millis(1000).await;
    }
}

