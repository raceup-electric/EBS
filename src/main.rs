#![no_std]
#![no_main]

use brake::BrakeController;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::peripherals::{ADC1, ADC2, PA1, PA2};
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_time::Timer;
use defmt::*;
use static_cell::StaticCell;
use embassy_sync::signal::Signal;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use panic_probe as _;
use defmt_rtt as _;
use embassy_sync::channel::Channel;


mod pressure;
mod brake;
mod can_management;
mod driverless;

use can_management::can_controller::CanController;
use can_sevice::init_can_service;

use pressure::sensor::Sensor;
use pressure::pressure_sensor::{BrakePressureSensor, brake_pressure_monitor};
use brake::{BrakeSignal, BrakeStatus};

use core::sync::atomic::AtomicU8;

static MISSION: AtomicU8 = AtomicU8::new(Mission::Null as u8);
static STATUS: AtomicU8 = AtomicU8::new(DVRunStatus::Null as u8);

static BRAKE_PRESSURE_SENSOR: StaticCell<BrakePressureSensor> = StaticCell::new();
static BRAKE_CONTROLLER: StaticCell<BrakeController> = StaticCell::new();

static BRAKE_SIGNAL: Signal<CriticalSectionRawMutex, brake::BrakeSignal> = Signal::new();

static CAN_MSG_CHANNEL: Channel<CriticalSectionRawMutex, Messages, 10> = Channel::new();

static CAN: StaticCell<Mutex<CriticalSectionRawMutex, CanController>> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let brake_pressure_sensor = BRAKE_PRESSURE_SENSOR.init(BrakePressureSensor::new(
        Sensor::new(Adc::new(p.ADC1), p.PA1),
        Sensor::new(Adc::new(p.ADC2), p.PA2),
    ));
    spawner.spawn(brake_pressure_monitor(brake_pressure_sensor)).unwrap();
    
    let brake_controller = BRAKE_CONTROLLER.init(BrakeController::new(p.PC6,p.PC7));
    spawner.spawn(brake_control_task(brake_controller));
    

    let can_peripherals = init_can_peripherals(&mut p);    
    init_can_service(&spawner, can_peripherals).await;
}

