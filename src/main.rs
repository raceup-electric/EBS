#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::adc::Adc;
use embassy_stm32::can::filter::BankConfig;
use embassy_stm32::can::frame::Timestamp;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Timer;
use static_cell::StaticCell;

use crate::can_management::messages::EbsStatusAsbCheck;
use panic_probe as _;
use defmt_rtt as _;

// use panic_probe as _;
use embassy_stm32::can::filter::ListEntry16;
use embassy_stm32::can::{CanRx, CanTx, Fifo, Frame, Id, StandardId};
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Ticker};

mod brake;
mod can_management;
mod config;
mod tank_pressure;

use brake::BrakeController;
use brake::BrakeSignal;
use can_management::can_controller::CanController;
use can_management::messages::{
    CarMissionStatus, CarMissionStatusMission, CheckAsbReq, EbsBrakeReq, EbsStatus,
    HydraulicPressure,
};
use config::pressure_thresholds::*;
use tank_pressure::pressure_sensor::{TankPressureSensor, tank_pressure_monitor};
use tank_pressure::sensor::Sensor;

static TANK_PRESSURE_SENSOR: StaticCell<TankPressureSensor> = StaticCell::new();
static BRAKE_CONTROLLER: StaticCell<BrakeController> = StaticCell::new();

static BRAKE_SIGNAL: Signal<CriticalSectionRawMutex, brake::BrakeSignal> = Signal::new();
static CAN_WRITER: Channel<CriticalSectionRawMutex, Frame, 20> = Channel::new();

// Signal to update status

static MISSION: Signal<CriticalSectionRawMutex, (CarMissionStatusMission, Timestamp)> =
    Signal::new();
pub static TANK_PRESSURE: Signal<CriticalSectionRawMutex, TankPressure> = Signal::new();
pub static TANK_PRESSURE_SHARED: Mutex<CriticalSectionRawMutex, RefCell<TankPressure>> =
    Mutex::new(RefCell::new(TankPressure::new(0.0, 0.0)));
static BRAKE_PRESSURE: Signal<CriticalSectionRawMutex, (f32, f32)> = Signal::new();
static BRAKE_REQ: Signal<CriticalSectionRawMutex, (bool, Timestamp)> = Signal::new();
static ASB_CHECK_REQ: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    let tank_pressure_sensor = TANK_PRESSURE_SENSOR.init(TankPressureSensor::new(
        Sensor::new(Adc::new(p.ADC1), p.PA1),
        Sensor::new(Adc::new(p.ADC2), p.PA2),
    ));
    spawner
        .spawn(tank_pressure_monitor(tank_pressure_sensor))
        .unwrap();

    let brake_controller = BRAKE_CONTROLLER.init(BrakeController::new(p.PC6, p.PC7));
    spawner.spawn(brake_control_task(brake_controller)).unwrap();

    let (mut can, _rx1, _tx1) =
        CanController::new_can2(p.CAN2, p.PB12, p.PB13, 500_000, p.CAN1, p.PA11, p.PA12).await;

    let (can_tx, can_rx) = can.can.split();

    can.can.modify_filters().enable_bank(
        0,
        Fifo::Fifo1,
        BankConfig::List16([
            ListEntry16::data_frames_with_id(unwrap!(StandardId::new(
                HydraulicPressure::MESSAGE_ID as u16
            ))),
            ListEntry16::data_frames_with_id(unwrap!(StandardId::new(
                CarMissionStatus::MESSAGE_ID as u16
            ))),
            ListEntry16::data_frames_with_id(unwrap!(StandardId::new(0x1))),
            ListEntry16::data_frames_with_id(unwrap!(StandardId::new(0x1))),
        ]),
    );

    spawner.spawn(can_writer(can_tx)).unwrap();
    spawner.spawn(can_reader(can_rx)).unwrap();

    let mut global_status = GlobalStatus::new();
    let mut main_status = MainStatus::new();

    let mut ticker = Ticker::every(Duration::from_millis(10));

    loop {
        ticker.next().await;
        global_status.update();

        if main_status.phase != Phase::Zero {
            if !global_status.mission.is_dv() {
                global_status.reset();
                main_status.reset();
                continue;
            }
            main_status.update(&global_status.tank_pressure, &global_status.brake_pressure);
            send_ebs_status_msg(&main_status, &global_status.tank_pressure).await;
        }

        match main_status.phase {
            Phase::Zero => {
                if global_status.mission.is_dv() {
                    main_status.set_phase(Phase::One);
                    main_status.phase_click_counter = 0;
                } else {
                    main_status.phase_click_counter += 1;
                }
            }
            Phase::One => {
                if global_status.asb_check_req == true {
                    main_status.asb_check_status = EbsStatusAsbCheck::Ongoing;
                    main_status.set_phase(Phase::Two(PhaseTwo::FirstTankBraking));
                    BRAKE_SIGNAL.signal(BrakeSignal::TankOneCheck);
                    main_status.phase_click_counter = 0;
                } else {
                    main_status.phase_click_counter += 1;
                }
            }
            Phase::Two(subphase) => match subphase {
                PhaseTwo::FirstTankBraking => {
                    if main_status.phase_click_counter == 100 {
                        main_status.set_phase(Phase::Two(PhaseTwo::CheckFirstTank));
                        main_status.phase_click_counter = 0;
                    } else {
                        main_status.phase_click_counter += 1;
                    }
                }
                PhaseTwo::CheckFirstTank => {
                    if main_status.phase_click_counter == 0 && main_status.brake_consistency {
                        BRAKE_SIGNAL.signal(BrakeSignal::Release);
                        main_status.set_phase(Phase::Two(PhaseTwo::EmptyFirstTank));
                    } else {
                        main_status.asb_check_status = EbsStatusAsbCheck::Failed;
                        main_status.system_check = false;
                        main_status.phase_click_counter += 1;
                    }
                }
                PhaseTwo::EmptyFirstTank => {
                    if main_status.phase_click_counter == 500 {
                        if check_brake_released(&global_status.brake_pressure) {
                            BRAKE_SIGNAL.signal(BrakeSignal::TankTwoCheck);
                            main_status.set_phase(Phase::Two(PhaseTwo::SecondTankBraking));
                            main_status.phase_click_counter = 0;
                        } else {
                            main_status.asb_check_status = EbsStatusAsbCheck::Failed;
                            main_status.system_check = false;
                            main_status.phase_click_counter += 1;
                        }
                    } else {
                        main_status.phase_click_counter += 1;
                    }
                }
                PhaseTwo::SecondTankBraking => {
                    if main_status.phase_click_counter == 100 {
                        main_status.set_phase(Phase::Two(PhaseTwo::CheckSecondTank));
                        main_status.phase_click_counter = 0;
                    } else {
                        main_status.phase_click_counter += 1;
                    }
                }
                PhaseTwo::CheckSecondTank => {
                    if main_status.phase_click_counter == 0 && main_status.brake_consistency {
                        main_status.set_phase(Phase::Three);
                        main_status.phase_click_counter = 0;
                        main_status.asb_check_status = EbsStatusAsbCheck::Passed;
                    } else {
                        main_status.asb_check_status = EbsStatusAsbCheck::Failed;
                        main_status.system_check = false;
                        main_status.phase_click_counter += 1;
                    }
                }
            },
            Phase::Three => {
                if global_status.brake_req != main_status.brake_engaged {
                    match global_status.brake_req {
                        true => {
                            main_status.ts_last_brake = Timestamp::now();
                            BRAKE_SIGNAL.signal(BrakeSignal::Engage)
                        }
                        false => BRAKE_SIGNAL.signal(BrakeSignal::Release),
                    }
                }

                //TODO: di sicuro mancano dei controlli per andare in emergency
                if global_status.last_vcu_core2.elapsed() > Duration::from_millis(200)
                    || global_status.last_embedded.elapsed() > Duration::from_millis(200)
                    || (global_status.brake_req && main_status.ts_last_brake.elapsed() > Duration::from_millis(200) && !main_status.brake_consistency)
                    || (!main_status.tank_pressure_ok && main_status.ts_last_brake.elapsed() < Duration::from_millis(200))
                {
                    main_status.system_check = false;
                    BRAKE_SIGNAL.signal(BrakeSignal::DoubleBrake);
                    main_status.set_phase(Phase::Four);
                }
            }
            Phase::Four => {}
        }
    }
}

//informazioni ricevute esternamente al main
struct GlobalStatus {
    mission: CarMissionStatusMission,
    tank_pressure: TankPressure,
    brake_pressure: BrakePressure,
    speed: f32,
    asb_check_req: bool,
    brake_req: bool,
    last_vcu_core2: Timestamp,
    last_embedded: Timestamp,
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
            mission: CarMissionStatusMission::None,
            tank_pressure: TankPressure::new(0.0, 0.0),
            brake_pressure: BrakePressure::new(),
            speed: 0.0,
            asb_check_req: false,
            brake_req: false,
            last_vcu_core2: Timestamp::now(),
            last_embedded: Timestamp::now(),
        }
    }

    pub fn reset(&mut self) {
        self.mission = CarMissionStatusMission::None;
        self.tank_pressure = TankPressure::new(0.0, 0.0);
        self.brake_pressure = BrakePressure::new();
        self.speed = 0.0;
        self.brake_req = false;
        self.asb_check_req = false;
        self.last_vcu_core2 = Timestamp::now();
        self.last_embedded = Timestamp::now();
    }

    pub fn update(&mut self) {
        if let Some(new_mission) = MISSION.try_take() {
            self.mission = new_mission.0;
            self.last_vcu_core2 = new_mission.1;
        }
        if let Some(new_tank_pressure) = TANK_PRESSURE.try_take() {
            self.tank_pressure = new_tank_pressure
        }
        if let Some(new_brake_pressure) = BRAKE_PRESSURE.try_take() {
            self.brake_pressure.set_front_rear(new_brake_pressure);
        }
        if let Some(_asb_check_req) = ASB_CHECK_REQ.try_take() {
            self.asb_check_req = true;
        }
        if let Some(new_brake_req) = BRAKE_REQ.try_take() {
            self.brake_req = new_brake_req.0;
            self.last_embedded = new_brake_req.1;
        }
    }
}

impl Format for EbsStatusAsbCheck {
    fn format(&self, fmt: Formatter) {
        match self {
            EbsStatusAsbCheck::NotRequested => {
                defmt::write!(fmt, "not requested")
            }
            EbsStatusAsbCheck::Ongoing => {
                defmt::write!(fmt, "ongoing")
            }
            EbsStatusAsbCheck::Passed => {
                defmt::write!(fmt, "passed")
            }
            EbsStatusAsbCheck::Failed => {
                defmt::write!(fmt, "failed")
            }
            &EbsStatusAsbCheck::_Other(_) => {
                defmt::write!(fmt, "other")
            }
        }
    }
}

//da intendere come le informazioni interne alla macchina a stati durante l'esecuzione
//la differenza primaria tra MainS2tatus e GlobalStatus è che i valori appartenti alla prima sono computati e modificati dalla main task mentre i valori della seconda vengono solo letti
struct MainStatus {
    system_check: bool,
    phase: Phase,
    phase_click_counter: u32,
    asb_check_status: EbsStatusAsbCheck,
    brake_engaged: bool,
    brake_consistency: bool,
    ts_last_brake: Timestamp,
    tank_pressure_ok: bool,
}

impl MainStatus {
    pub fn new() -> Self {
        Self {
            system_check: false,
            phase: Phase::Zero,
            phase_click_counter: 0,
            asb_check_status: EbsStatusAsbCheck::NotRequested,
            brake_engaged: false,
            brake_consistency: false,
            ts_last_brake: Timestamp::now(),
            tank_pressure_ok: false,
        }
    }

    pub fn update(&mut self, tank_press: &TankPressure, brake_press: &BrakePressure) {
        self.brake_consistency = check_brake_consistency(&brake_press);
        self.brake_engaged =
            brake_press.front > MIN_FRONT_PRESS || brake_press.rear > MIN_REAR_PRESS;
        self.tank_pressure_ok = check_tank_pressure(tank_press);
    }

    #[allow(dead_code)]
    pub fn set_phase(&mut self, new_phase: Phase) {
        self.phase = new_phase;
        self.phase_click_counter = 0;
        match new_phase {
            Phase::Zero => {
                self.reset();
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.system_check = false;
        self.phase = Phase::Zero;
        self.phase_click_counter= 0;
        self.asb_check_status = EbsStatusAsbCheck::NotRequested;
        self.brake_engaged = false;
        self.brake_consistency = false;
        self.ts_last_brake = Timestamp::now();
        self.tank_pressure_ok = false;
    }
}

impl CarMissionStatusMission {
    pub fn is_dv(&self) -> bool {
        match &self {
            CarMissionStatusMission::DvInspection
            | CarMissionStatusMission::DvEbsTest
            | CarMissionStatusMission::DvTrackdrive
            | CarMissionStatusMission::DvAutocross
            | CarMissionStatusMission::DvSkidpad
            | CarMissionStatusMission::DvAcceleration => true,
            _ => false,
        }
    }
}

#[repr(u16)]
#[allow(dead_code)]
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

#[allow(dead_code)]
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
    pub const fn new(t1: f32, t2: f32) -> Self {
        Self {
            tank_one_pressure: t1,
            tank_two_pressure: t2,
        }
    }

    pub fn set(&mut self, t1: f32, t2: f32) {
        self.tank_one_pressure = t1;
        self.tank_two_pressure = t2;
    }

    pub fn values(&self) -> (f32, f32) {
        (self.tank_one_pressure, self.tank_two_pressure)
    }

    pub fn avg(&self) -> f32 {
        (self.tank_one_pressure + self.tank_two_pressure) * 0.5
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
    if let Ok(main_status_msg) = EbsStatus::new(
        main_status.system_check,
        main_status.asb_check_status.into(),
        main_status.brake_engaged,
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

#[embassy_executor::task]
async fn brake_control_task(controller: &'static mut BrakeController) {
    loop {
        let sig = BRAKE_SIGNAL.wait().await;
        let (t1, t2) = TANK_PRESSURE_SHARED.lock(|cell| cell.borrow().values());
        controller.handle_signal(sig, t1, t2);
        Timer::after(Duration::from_millis(1)).await;
    }
}

#[embassy_executor::task]
async fn can_writer(mut tx: CanTx<'static>) {
    loop {
        let frame = CAN_WRITER.receive().await;
        tx.write(&frame).await;
        Timer::after(Duration::from_micros(100)).await;
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
                    HydraulicPressure::MESSAGE_ID => {
                        if let Ok(msg) = HydraulicPressure::try_from(payload) {
                            BRAKE_PRESSURE
                                .signal((msg.press_front() as f32, msg.press_rear() as f32));
                        }
                    }
                    CarMissionStatus::MESSAGE_ID => {
                        if let Ok(msg) = CarMissionStatus::try_from(payload) {
                            MISSION.signal((msg.mission(), frame.ts));
                        }
                    }
                    CheckAsbReq::MESSAGE_ID => {
                        ASB_CHECK_REQ.signal(());
                    }
                    EbsBrakeReq::MESSAGE_ID => {
                        if let Ok(msg) = EbsBrakeReq::try_from(payload) {
                            BRAKE_REQ.signal((msg.req(), frame.ts))
                        }
                    }
                    _ => {}
                }
            }
            Err(_) => {} //info!("No messages")
        }
    }
}
