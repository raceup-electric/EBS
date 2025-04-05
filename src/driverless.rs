use core::sync::atomic::{AtomicU8, Ordering};
use embassy_sync::signal::Signal;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mission {
    Null,
    Manual,
    Driveless,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DVRunState {
    Null,
    ReadyToDrive,
    Run,
    EndRun,
}

pub fn set_mission(mission: Mission) {
    MISSION.store(mission as u8, Ordering::Relaxed);
}

pub fn get_mission() -> Mission {
    match MISSION.load(Ordering::Relaxed) {
        1 => Mission::Manual,
        2 => Mission::Driveless,
        _ => Mission::Null,
    }
}

pub fn set_driverless_run(part: DriverlessRunState) {
    DRIVERLESS_RUN.store(part as u8, Ordering::Relaxed);
}

pub fn get_driverless_run() -> DriverlessRunState {
    match DRIVERLESS_RUN.load(Ordering::Relaxed) {
        1 => DriverlessRunState::ReadyToDrive,
        2 => DriverlessRunState::Run,
        3 => DriverlessRunState::EndRun,
        _ => DriverlessRunState::Null,
    }
}
