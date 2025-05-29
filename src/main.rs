#![no_std]
#![no_main]

use brake::BrakeController;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::pac::crc::regs::Cr;
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
use embassy_time::Duration;
use embassy_stm32::can::{CanTx, CanRx};
use crate::can_management::CanFrame;
use crate::can_management::messages::Messages;

mod tank_pressure;
mod brake;
mod can_management;
mod config;

use can_management::can_controller::CanController;

use tank_pressure::sensor::Sensor;
use tank_pressure::pressure_sensor::{TankPressureSensor, tank_pressure_monitor};
use brake::{BrakeSignal, BrakeStatus};
use config::pressure_thresholds::*;

static TANK_PRESSURE_SENSOR: StaticCell<TankPressureSensor> = StaticCell::new();
static BRAKE_CONTROLLER: StaticCell<BrakeController> = StaticCell::new();
static CAN: StaticCell<CanController> = StaticCell::new();

static BRAKE_SIGNAL: Signal<CriticalSectionRawMutex, brake::BrakeSignal> = Signal::new();

static CAN_WRITER: Channel<CriticalSectionRawMutex, (u16, [u8; 8]), 20> = Channel::new();


// Signal to update status

static MISSION : Signal<CriticalSectionRawMutex, Mission> = Signal::new();
pub static TANK_STATUS : Signal<CriticalSectionRawMutex, TankStatus> = Signal::new();
static SPEED : Signal<CriticalSectionRawMutex, f32> = Signal::new();
static BRAKE_PRESSURE : Signal<CriticalSectionRawMutex, (f32, f32)> = Signal::new();
static ERROR : Signal<CriticalSectionRawMutex, bool> = Signal::new();
static GO : Signal<CriticalSectionRawMutex, ()> = Signal::new();


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let tank_pressure_sensor = TANK_PRESSURE_SENSOR.init (TankPressureSensor::new(
        Sensor::new(Adc::new(p.ADC1), p.PA1),
        Sensor::new(Adc::new(p.ADC2), p.PA2),
    ));
    spawner.spawn(tank_pressure_monitor(tank_pressure_sensor)).unwrap();
    
    let brake_controller = BRAKE_CONTROLLER.init(BrakeController::new(p.PC6,p.PC7));
    spawner.spawn(brake_control_task(brake_controller)).unwrap();
    

    let mut can = CanController::new_can2(p.CAN2, p.PB12, p.PB13, 500_000, p.CAN1, p.PA11, p.PA12).await;
    let (can_tx, can_rx) = can.can.split();

    spawner.spawn(can_writer(can_tx)).unwrap();

    let mut car_status = CarStatus::new();
    let mut ebs_status = EbsStatus::new();
    
    loop{
        car_status.update();
        if car_status.error == true {
            ebs_status.set_phase(Phase::Zero);
        }
        //todo: add await for each phase
        match ebs_status.phase {
            Phase::Zero  => {
                if car_status.mission.is_dv() {
                    ebs_status.set_phase(Phase::One)
                }
            }
            Phase::One   => {
                if car_status.request == Request::AsbCheck {
                    ebs_status.set_phase(Phase::Two(PhaseTwo::FirstTankCheck))
                }
                else {
                let tank_press_validation = is_tank_pressure_valid(& car_status.tank_status);
                let tank_brake_consistency = check_tank_brake_pressure_consistency(& car_status.tank_status, & car_status.brake_pressure);
                //send message for validation
                }
            }
            Phase::Two(subphase)  => match subphase {
                
                PhaseTwo::FirstTankCheck  => {
                    BRAKE_SIGNAL.signal(BrakeSignal::TankOneCheck);
                    ebs_status.set_phase(Phase::Two(PhaseTwo::SecondTankCheck));
                }
                PhaseTwo::SecondTankCheck => {
                    //pressure validation
                    BRAKE_SIGNAL.signal(BrakeSignal::TankTwoCheck);
                    ebs_status.set_phase(Phase::Two(PhaseTwo::SecondTankCheck));               
                }
                PhaseTwo::SendValidation  => {
                    //pressure validation
                    //send exit EBS_check
                    ebs_status.set_phase(Phase::Three);  
                }               
                
            }
            Phase::Three => {
                //send validation brake and tanks
                // wait for MCU to validate
                ebs_status.set_phase(Phase::Four);

                
            }
            Phase::Four  => {
                GO.wait().await;
                BRAKE_SIGNAL.signal(BrakeSignal::Release);
                ebs_status.set_phase(Phase::Five);
                
            }
            Phase::Five  => {
                //spawn continuos monitoring or use the spawned one
                //send value
                //check for brake request
                //check for end run
                
            }
        }
    }
}

//informazioni ricevute esternamente dalla macchina a stati
struct CarStatus {
    mission:             Mission,
    tank_status:         TankStatus,
    brake_pressure:      BrakePressure,
    speed:               f32,
    request:             Request,
    error:               bool,
    
}
struct BrakePressure {
    front: f32,
    rear: f32
}

impl BrakePressure {
    pub fn new() -> Self {
        Self{
            front: 0.0,
            rear:  0.0
        }
    }
    pub fn set_front_rear(&mut self, new_press :(f32, f32)) {
        self.front = new_press.0;
        self.rear = new_press.1;
    }
}


impl CarStatus {
    pub fn new() -> Self {
        Self {
            mission:              Mission::None,
            tank_status:          TankStatus::new(0.0,0.0,true, true),
            brake_pressure:       BrakePressure::new(),
            speed:                0.0,
            request:              Request::None,
            error:                false,

        }
    }

    pub async fn update(&mut self) {
        if let Some(new_mission) = MISSION.try_take() {
            self.mission = new_mission;
        }
        if let Some(new_tank_status) = TANK_STATUS.try_take() {
            self.tank_status = new_tank_status
        }
        if let Some(new_speed) = SPEED.try_take() {
            self.speed = new_speed;
        }
        if let Some(new_brake_pressure) = BRAKE_PRESSURE.try_take() {
            self.brake_pressure.set_front_rear(new_brake_pressure);
        }
        if let Some(new_error) = ERROR.try_take() {
            self.error = new_error;
        }
    }
}
//da intendere come le informazioni interne alla macchina a stati durante l'esecuzione
//la differenza primaria tra EbsStatus e CarStatus è che i valori appartenti alla prima sono computati e modificati dalla macchina a stati mentre i valori della seconda vengono solo letti
struct EbsStatus {
    phase:   Phase,
    tank_validation: bool,
    tank_brake_consistency: bool,
    asb_check: bool,
    brake_engaged: bool,
}

impl EbsStatus {
    pub fn new() -> Self {
        Self {
            phase: Phase::Zero,
            tank_validation: false,
            tank_brake_consistency: false,
            asb_check: false,
            brake_engaged: false,
        }
    }
    pub fn set_phase(&mut self, new_phase: Phase) {
        self.phase = new_phase;
    }

}


enum Mission {
    None,
    Manual,
    DVinspection,
    EBStest,
    DVtrackdrive,
    DVautocross,
    DVskidpad,
    DVacceleration,
}

impl Mission{
    pub fn is_dv(&self) -> bool{
        match &self{
            (Mission::DVinspection |
            Mission::EBStest       |
            Mission::DVtrackdrive  |
            Mission::DVautocross   |
            Mission::DVskidpad     |
            Mission::DVacceleration) => true,
            _ => false
        }
        
    }
}

#[derive(Copy, Clone)]
enum Phase {
    Zero,  // not in dv mission
    One,   // sdc open, validation brake
    Two(PhaseTwo),   // ASB check
    Three, // brake engaged, validation brake
    Four,  // Ready to Drive, wait for GO
    Five   // running: Continous Monitoring and Brake Service
}

#[derive(Copy, Clone)]
enum PhaseTwo {
    FirstTankCheck,
    SecondTankCheck,
    SendValidation,
}

struct TankStatus{
    tank_one_pressure: f32,
    tank_two_pressure: f32,
    sensor_one_sanity: bool,
    sensor_two_sanity: bool
}

impl TankStatus{
    pub fn new(t1: f32, t2: f32, s1: bool, s2: bool) -> Self {
        Self {
            tank_one_pressure: t1,
            tank_two_pressure: t2,
            sensor_one_sanity: s1,
            sensor_two_sanity: s2,
        }
    }
}

#[derive(PartialEq)]
enum Request{
    None,
    Brake,
    AsbCheck,
    AsbCheckAck,
}
pub fn is_tank_pressure_valid (tank_status: &TankStatus) -> bool{
    tank_status.tank_one_pressure > MIN_TANK_PRESS &&
    tank_status.tank_one_pressure < MAX_TANK_PRESS &&
    tank_status.tank_two_pressure > MIN_TANK_PRESS &&
    tank_status.tank_two_pressure < MAX_TANK_PRESS

}

pub fn check_tank_brake_pressure_consistency(tank_status: &TankStatus, brake_press: &BrakePressure) -> bool {
    brake_press.front > FRONT_PRESS_MULT * tank_status.tank_one_pressure &&
    brake_press.front > FRONT_PRESS_MULT * tank_status.tank_two_pressure &&
    brake_press.rear  > REAR_PRESS_MULT * tank_status.tank_one_pressure &&
    brake_press.rear  > REAR_PRESS_MULT * tank_status.tank_two_pressure
}

pub fn send_phase_one_validation(tank_press_validation: bool, tank_brake_consistency_check: bool, car_status: &CarStatus){
    //generate message EBS_Status for phase one
}

#[embassy_executor::task]
async fn brake_control_task(controller: &'static mut BrakeController) {
    loop {
        let sig = BRAKE_SIGNAL.wait().await;
        controller.handle_signal(sig);
        Timer::after(Duration::from_millis(10)).await;
    }
}



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