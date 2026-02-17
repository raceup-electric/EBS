#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_stm32::can::filter::BankConfig;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use static_cell::StaticCell;

use crate::can_management::messages::CarMissionStatusMissionStatus;
use crate::usb_serial::usb::Serial;
// use panic_probe as _;
// use defmt_rtt as _;

use defmt::info;
// use panic_probe as _;
use embassy_stm32::can::filter::ListEntry16;
use embassy_stm32::can::{CanRx, CanTx, Fifo, Frame, Id, StandardId};
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Ticker};

mod brake;
mod can_management;
mod config;
mod tank_pressure;
mod usb_serial;

use brake::BrakeController;
use brake::BrakeSignal;
use can_management::can_controller::CanController;
use can_management::messages::{
    CarMission, CarMissionStatus, CarMissionStatusAsStatus, CarMissionStatusMission, CarStatus,
    CheckAsbReq, EbsBrakeReq, EbsStatus, HydraulicPressure, ResGo,
};
use config::pressure_thresholds::*;
use tank_pressure::pressure_sensor::{TankPressureSensor, tank_pressure_monitor};
use tank_pressure::sensor::Sensor;
use usb_serial::prepare_config;

static TANK_PRESSURE_SENSOR: StaticCell<TankPressureSensor> = StaticCell::new();
static BRAKE_CONTROLLER: StaticCell<BrakeController> = StaticCell::new();

static BRAKE_SIGNAL: Signal<CriticalSectionRawMutex, brake::BrakeSignal> = Signal::new();
static CAN_WRITER: Channel<CriticalSectionRawMutex, Frame, 20> = Channel::new();

// Signal to update status

static MISSION: Signal<CriticalSectionRawMutex, Mission> = Signal::new();
pub static TANK_STATUS: Signal<CriticalSectionRawMutex, TankPressure> = Signal::new();
static SPEED: Signal<CriticalSectionRawMutex, f32> = Signal::new();
static BRAKE_PRESSURE: Signal<CriticalSectionRawMutex, (f32, f32)> = Signal::new();
static ERROR: Signal<CriticalSectionRawMutex, ()> = Signal::new();
static BRAKE_REQ: Signal<CriticalSectionRawMutex, bool> = Signal::new();
static ASB_CHECK_REQ: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(prepare_config());

    let tank_pressure_sensor = TANK_PRESSURE_SENSOR.init(TankPressureSensor::new(
        Sensor::new(Adc::new(p.ADC1), p.PA1),
        Sensor::new(Adc::new(p.ADC2), p.PA2),
    ));
    spawner
        .spawn(tank_pressure_monitor(tank_pressure_sensor))
        .unwrap();

    let brake_controller = BRAKE_CONTROLLER.init(BrakeController::new(p.PC6, p.PC7));
    spawner.spawn(brake_control_task(brake_controller)).unwrap();

    let (mut can, rx1, tx1) =
        CanController::new_can2(p.CAN2, p.PB12, p.PB13, 500_000, p.CAN1, p.PA11, p.PA12).await;

    Serial::init(p.USB_OTG_FS, tx1, rx1, &spawner);

    let (can_tx, can_rx) = can.can.split();

    can.can.modify_filters().enable_bank(
        0,
        Fifo::Fifo1,
        BankConfig::List16([
            ListEntry16::data_frames_with_id(unwrap!(StandardId::new(
                CarStatus::MESSAGE_ID as u16
            ))),
            ListEntry16::data_frames_with_id(unwrap!(StandardId::new(
                HydraulicPressure::MESSAGE_ID as u16
            ))),
            ListEntry16::data_frames_with_id(unwrap!(StandardId::new(
                CarMissionStatus::MESSAGE_ID as u16
            ))),
            ListEntry16::data_frames_with_id(unwrap!(StandardId::new(0x1))),
        ]),
    );

    spawner.spawn(can_writer(can_tx)).unwrap();
    spawner.spawn(can_reader(can_rx)).unwrap();

    let mut global_status = GlobalStatus::new();
    let mut main_status = MainStatus::new();

    let mut time = embassy_time::Instant::now().as_millis();

    let mut ticker = Ticker::every(Duration::from_millis(50));

    loop {
        ticker.next().await;
        global_status.update();

        if main_status.phase != Phase::Zero {
            if !global_status.mission.is_dv() {
                global_status.reset();
                main_status.reset();
                continue;
            }
            main_status.click_counter += 1;
            main_status.update(&global_status.tank_status, &global_status.brake_pressure);
            send_ebs_status_msg(&main_status, &global_status.tank_status).await;
        }

        // debug usb
        if embassy_time::Instant::now().as_millis() - time > 2000u64 {
            info!(
                "** GLOBAL STATUS **\r\n    Mission: {}\r\n    Tank Status -> T1 {}, T2 {}\r\n    Brake Pressure Front: {}, Brake Pressure Rear: {}\r\n    ASB Brake Request: {}\r\n    Brake Request: {}\r\n    Error: {}\r\n",
                global_status.mission.as_raw(),
                global_status.tank_status.tank_one_pressure,
                global_status.tank_status.tank_two_pressure,
                global_status.brake_pressure.front,
                global_status.brake_pressure.rear,
                global_status.asb_check_req,
                global_status.brake_req,
                global_status.error
            );
            info!(
                "** MAIN STATUS**\r\n    Phase: {}\r\n    Click Counter: {}\r\n    Tank Validation: {}\r\n    ASB Check: {}\r\n    Brake engaged: {}\r\n    Brake Consistency: {}\r\n    Internal Error: {}\r\n",
                main_status.phase.value(),
                main_status.click_counter,
                main_status.tank_validation,
                main_status.asb_check,
                main_status.brake_engaged,
                main_status.brake_consistency,
                main_status.internal_error
            );
            time = embassy_time::Instant::now().as_millis();
        }

        match main_status.phase {
        }
    }
}

//informazioni ricevute esternamente al main
struct GlobalStatus {
    mission: Mission,
    tank_status: TankPressure,
    brake_pressure: BrakePressure,
    speed: f32,
    asb_check_req: bool,
    brake_req: bool,
}

#[derive(Debug)]
struct BrakePressure {
    front: f32,
    rear: f32,
}

impl BrakePressure {
    pub fn new() -> Self {
        Self {
            front: 0.0,
            rear: 0.0,
        }
    }
    pub fn set_front_rear(&mut self, new_press: (f32, f32)) {
        self.front = new_press.0;
        self.rear = new_press.1;
    }
}

impl GlobalStatus {
    pub fn new() -> Self {
        Self {
            mission: Mission::None,
            tank_status: TankPressure::new(0.0, 0.0),
            brake_pressure: BrakePressure::new(),
            speed: 0.0,
            asb_check_req: false,
            brake_req: false,
        }
    }

    pub fn reset(&mut self) {
        self.mission = Mission::None;
        self.tank_status = TankPressure::new(0.0, 0.0);
        self.brake_pressure = BrakePressure::new();
        self.speed = 0.0;
        self.brake_req = false;
        self.asb_check_req = false;
    }

    pub fn update(&mut self) {
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
        if let Some(_asb_check_req) = ASB_CHECK_REQ.try_take() {
            self.asb_check_req = true;
        }
        if let Some(new_brake_req) = BRAKE_REQ.try_take() {
            self.brake_req = new_brake_req;
        }
    }
}

//da intendere come le informazioni interne alla macchina a stati durante l'esecuzione
//la differenza primaria tra MainS2tatus e GlobalStatus è che i valori appartenti alla prima sono computati e modificati dalla main task mentre i valori della seconda vengono solo letti
#[derive(Debug)]
struct MainStatus {
    phase: Phase,
    phase_click_counter: u32,
    asb_check_passed: bool,
    brake_engaged: bool,
    brake_consistency: bool,
}

impl MainStatus {
    pub fn new() -> Self {
        Self {
            phase: Phase::Zero,
            phase_click_counter: 0,
            asb_check_passed: false,
            brake_engaged: false,
            brake_consistency: false,
        }
    }

    pub fn update(&mut self, tank_status: &TankPressure, brake_press: &BrakePressure) {
        self.tank_validation = check_tank_pressure(&tank_status);
        self.brake_consistency = check_brake_consistency(&brake_press);
    }

    pub fn set_phase(&mut self, new_phase: Phase) {
        self.phase = new_phase;
        self.click_counter = 0;
        match new_phase {
            Phase::Zero => {
                self.reset();
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.phase = Phase::Zero;
        self.tank_validation = false;
        self.asb_check = false;
        self.brake_engaged = false;
        self.brake_consistency = false;
    }
}

impl CarMissionStatusMission {
    pub fn is_dv(&self) -> bool {
        match &self {
            CarMissionStatusMission::DvInspection |
            CarMissionStatusMission::DvEbsTest |
            CarMissionStatusMission::DvTrackdrive|
            CarMissionStatusMission::DvAutocross |
            CarMissionStatusMission::DvSkidpad |
            CarMissionStatusMission::DvAcceleration => true,
            _ => false,
        }
    }
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Phase {
    Zero = 0,          // not in dv mission
    One = 1,           // waiting for asb check
    Two(PhaseTwo) = 2, // ASB check
    Three,             // continuos monitoring
    Four,              // emergency
}

impl Phase {
    pub fn value(&self) -> u16 {
        match *self {
            Phase::Zero => 0,
            Phase::One => 1,
            Phase::Two(_) => 2,
            Phase::Three => 3,
            Phase::Four => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PhaseTwo {
    FirstTankBraking,
    CheckFirstTank,
    EmptyFirstTank,
    SecondTankBraking,
    CheckSecondTank,
}

pub struct TankPressure {
    tank_one_pressure: f32,
    tank_two_pressure: f32,
}

impl TankPressure {
    pub fn new(t1: f32, t2: f32) -> Self {
        Self {
            tank_one_pressure: t1,
            tank_two_pressure: t2,
        }
    }
}

fn check_tank_pressure(tank_press: &TankPressure) -> bool {
    tank_press.tank_one_pressure > MIN_TANK_PRESS
        && tank_press.tank_one_pressure < MAX_TANK_PRESS
        && tank_press.tank_two_pressure > MIN_TANK_PRESS
        && tank_press.tank_two_pressure < MAX_TANK_PRESS
}

fn check_brake_released(brake_press: &BrakePressure) -> bool {
    brake_press.front < 1.0 && brake_press.rear < 1.0
}

fn check_brake_consistency(brake_press: &BrakePressure) -> bool {
    brake_press.front > MIN_FRONT_PRESS && brake_press.rear > MIN_REAR_PRESS
}

async fn send_ebs_status_msg(main_status: &MainStatus, tank_status: &TankPressure) {
    let system_check = main_status.tank_validation;

    if let Ok(main_status_msg) = EbsStatus::new(
        system_check,
        main_status.asb_check_passed,
        main_status.brake_engaged,
        main_status.brake_consistency,
        tank_status.tank_one_pressure,
        tank_status.tank_two_pressure,
    ) {
        if let Ok(main_status_frame) =
            Frame::new_standard(EbsStatus::MESSAGE_ID as u16, main_status_msg.raw())
        {
            CAN_WRITER.send(main_status_frame).await;
        }
    }
}

fn engage_brake() {
    BRAKE_SIGNAL.signal(BrakeSignal::Engage);
}

fn release_brake() {
    BRAKE_SIGNAL.signal(BrakeSignal::Release);
}

#[embassy_executor::task]
async fn brake_control_task(controller: &'static mut BrakeController, tank_pressure: TankPressureSensor) {
    loop {
        let sig = BRAKE_SIGNAL.wait().await;
        controller.handle_signal(sig, tank_pressure.get_pressure_one(), tank_pressure.get_pressure_two());
        Timer::after(Duration::from_millis(1)).await;
    }
}

#[embassy_executor::task]
async fn can_writer(mut tx: CanTx<'static>) {
    loop {
        let frame = CAN_WRITER.receive().await;
        tx.write(&frame).await;
    }
}

#[embassy_executor::task]
async fn can_reader(mut rx: CanRx<'static>) {
    loop {
        match rx.read().await {
            Ok(frame) => {
                let id = match frame.frame.id() {
                    Id::Standard(id) => id.as_raw() as u32,
                    Id::Extended(id) => id.standard_id().as_raw() as u32,
                };
                let payload = frame.frame.data();
                match id {
                    CarStatus::MESSAGE_ID => {
                        if let Ok(msg) = CarStatus::try_from(payload) {
                            BRAKE_PRESSURE
                                .signal((msg.brake_front_press(), msg.brake_rear_press()));
                            SPEED.signal(msg.speed().into());
                        }
                    }
                    HydraulicPressure::MESSAGE_ID => {
                        if let Ok(msg) = HydraulicPressure::try_from(payload) {
                            BRAKE_PRESSURE
                                .signal((msg.press_front() as f32, msg.press_rear() as f32));
                        }
                    }

                    CarMissionStatus::MESSAGE_ID => {
                        if let Ok(msg) = CarMissionStatus::try_from(payload) {
                            MISSION.signal(msg.mission());
                        }
                    }

                    CheckAsbReq::MESSAGE_ID => {
                        ASB_CHECK_REQ.signal(());
                    }

                    EbsBrakeReq::MESSAGE_ID => {
                        if let Ok(msg) = EbsBrakeReq::try_from(payload) {
                            BRAKE_REQ.signal(msg.req())
                        }
                    }

                    _ => {}
                }
            }
            Err(_) => {} //info!("No messages")
        }
    }
}
