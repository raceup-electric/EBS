// Generated code!
#![allow(unused_comparisons, unreachable_patterns, unused_imports)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::useless_conversion, clippy::unnecessary_cast)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons, clippy::too_many_arguments)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"can2.dbc"`
//!
//! - Version: `Version("")`

use core::ops::BitOr;
use bitvec::prelude::*;
use embedded_can::{Id, StandardId, ExtendedId};

/// All messages
#[derive(Clone)]
pub enum Messages {
    /// ResGO
    ResGo(ResGo),
    /// EbsStatus
    EbsStatus(EbsStatus),
    /// CarMission
    CarMission(CarMission),
    /// PcuFault
    PcuFault(PcuFault),
    /// Paddle
    Paddle(Paddle),
    /// Driver
    Driver(Driver),
    /// BmsLv1
    BmsLv1(BmsLv1),
    /// BmsLv2
    BmsLv2(BmsLv2),
    /// BmsHv1
    BmsHv1(BmsHv1),
    /// BmsHv2
    BmsHv2(BmsHv2),
    /// Imu1
    Imu1(Imu1),
    /// Imu2
    Imu2(Imu2),
    /// Imu3
    Imu3(Imu3),
    /// IMUCalib
    ImuCalib(ImuCalib),
    /// Map
    Map(Map),
    /// CarStatus
    CarStatus(CarStatus),
    /// CarSettings
    CarSettings(CarSettings),
    /// CheckASBReq
    CheckAsbReq(CheckAsbReq),
    /// EbsBrakeReq
    EbsBrakeReq(EbsBrakeReq),
    /// ResStatus
    ResStatus(ResStatus),
    /// LapStart
    LapStart(LapStart),
    /// CarMissionStatus
    CarMissionStatus(CarMissionStatus),
    /// Temp1
    Temp1(Temp1),
    /// Temp2
    Temp2(Temp2),
    /// SuspRear
    SuspRear(SuspRear),
    /// RESERVED2
    Reserved2(Reserved2),
    /// SuspFront
    SuspFront(SuspFront),
    /// TempFrontR
    TempFrontR(TempFrontR),
    /// PressBrake
    PressBrake(PressBrake),
    /// InvVolt
    InvVolt(InvVolt),
    /// Pcu
    Pcu(Pcu),
    /// Calib
    Calib(Calib),
    /// CalibAck
    CalibAck(CalibAck),
    /// PcuSwControl
    PcuSwControl(PcuSwControl),
    /// PcuRfAck
    PcuRfAck(PcuRfAck),
    /// EmbeddedAliveCheck
    EmbeddedAliveCheck(EmbeddedAliveCheck),
    /// Lem
    Lem(Lem),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        
        let res = match id {
            ResGo::MESSAGE_ID => Messages::ResGo(ResGo::try_from(payload)?),
            EbsStatus::MESSAGE_ID => Messages::EbsStatus(EbsStatus::try_from(payload)?),
            CarMission::MESSAGE_ID => Messages::CarMission(CarMission::try_from(payload)?),
            PcuFault::MESSAGE_ID => Messages::PcuFault(PcuFault::try_from(payload)?),
            Paddle::MESSAGE_ID => Messages::Paddle(Paddle::try_from(payload)?),
            Driver::MESSAGE_ID => Messages::Driver(Driver::try_from(payload)?),
            BmsLv1::MESSAGE_ID => Messages::BmsLv1(BmsLv1::try_from(payload)?),
            BmsLv2::MESSAGE_ID => Messages::BmsLv2(BmsLv2::try_from(payload)?),
            BmsHv1::MESSAGE_ID => Messages::BmsHv1(BmsHv1::try_from(payload)?),
            BmsHv2::MESSAGE_ID => Messages::BmsHv2(BmsHv2::try_from(payload)?),
            Imu1::MESSAGE_ID => Messages::Imu1(Imu1::try_from(payload)?),
            Imu2::MESSAGE_ID => Messages::Imu2(Imu2::try_from(payload)?),
            Imu3::MESSAGE_ID => Messages::Imu3(Imu3::try_from(payload)?),
            ImuCalib::MESSAGE_ID => Messages::ImuCalib(ImuCalib::try_from(payload)?),
            Map::MESSAGE_ID => Messages::Map(Map::try_from(payload)?),
            CarStatus::MESSAGE_ID => Messages::CarStatus(CarStatus::try_from(payload)?),
            CarSettings::MESSAGE_ID => Messages::CarSettings(CarSettings::try_from(payload)?),
            CheckAsbReq::MESSAGE_ID => Messages::CheckAsbReq(CheckAsbReq::try_from(payload)?),
            EbsBrakeReq::MESSAGE_ID => Messages::EbsBrakeReq(EbsBrakeReq::try_from(payload)?),
            ResStatus::MESSAGE_ID => Messages::ResStatus(ResStatus::try_from(payload)?),
            LapStart::MESSAGE_ID => Messages::LapStart(LapStart::try_from(payload)?),
            CarMissionStatus::MESSAGE_ID => Messages::CarMissionStatus(CarMissionStatus::try_from(payload)?),
            Temp1::MESSAGE_ID => Messages::Temp1(Temp1::try_from(payload)?),
            Temp2::MESSAGE_ID => Messages::Temp2(Temp2::try_from(payload)?),
            SuspRear::MESSAGE_ID => Messages::SuspRear(SuspRear::try_from(payload)?),
            Reserved2::MESSAGE_ID => Messages::Reserved2(Reserved2::try_from(payload)?),
            SuspFront::MESSAGE_ID => Messages::SuspFront(SuspFront::try_from(payload)?),
            TempFrontR::MESSAGE_ID => Messages::TempFrontR(TempFrontR::try_from(payload)?),
            PressBrake::MESSAGE_ID => Messages::PressBrake(PressBrake::try_from(payload)?),
            InvVolt::MESSAGE_ID => Messages::InvVolt(InvVolt::try_from(payload)?),
            Pcu::MESSAGE_ID => Messages::Pcu(Pcu::try_from(payload)?),
            Calib::MESSAGE_ID => Messages::Calib(Calib::try_from(payload)?),
            CalibAck::MESSAGE_ID => Messages::CalibAck(CalibAck::try_from(payload)?),
            PcuSwControl::MESSAGE_ID => Messages::PcuSwControl(PcuSwControl::try_from(payload)?),
            PcuRfAck::MESSAGE_ID => Messages::PcuRfAck(PcuRfAck::try_from(payload)?),
            EmbeddedAliveCheck::MESSAGE_ID => Messages::EmbeddedAliveCheck(EmbeddedAliveCheck::try_from(payload)?),
            Lem::MESSAGE_ID => Messages::Lem(Lem::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// ResGO
///
/// - Standard ID: 32 (0x20)
/// - Size: 1 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct ResGo {
    raw: [u8; 1],
}

impl ResGo {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x20)});
    
    
    /// Construct new ResGO from values
    pub fn new(go_signal: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_go_signal(go_signal)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// go_signal
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn go_signal(&self) -> bool {
        self.go_signal_raw()
    }
    
    /// Get raw value of go_signal
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn go_signal_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of go_signal
    #[inline(always)]
    pub fn set_go_signal(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ResGo {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ResGo {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// EbsStatus
///
/// - Standard ID: 60 (0x3c)
/// - Size: 2 bytes
/// - Transmitter: EBS
#[derive(Clone, Copy)]
pub struct EbsStatus {
    raw: [u8; 2],
}

impl EbsStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x3c)});
    
    pub const PRESS_LEFT_TANK_MIN: f32 = 6_f32;
    pub const PRESS_LEFT_TANK_MAX: f32 = 10_f32;
    pub const PRESS_RIGHT_TANK_MIN: f32 = 6_f32;
    pub const PRESS_RIGHT_TANK_MAX: f32 = 10_f32;
    
    /// Construct new EbsStatus from values
    pub fn new(system_check: bool, press_left_tank: f32, press_right_tank: f32, sanity_left_sensor: bool, sanity_right_sensor: bool, asb_check: bool, brakes_engaged: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_system_check(system_check)?;
        res.set_press_left_tank(press_left_tank)?;
        res.set_press_right_tank(press_right_tank)?;
        res.set_sanity_left_sensor(sanity_left_sensor)?;
        res.set_sanity_right_sensor(sanity_right_sensor)?;
        res.set_asb_check(asb_check)?;
        res.set_brakes_engaged(brakes_engaged)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// system_check
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn system_check(&self) -> bool {
        self.system_check_raw()
    }
    
    /// Get raw value of system_check
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn system_check_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of system_check
    #[inline(always)]
    pub fn set_system_check(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// press_left_tank
    ///
    /// - Min: 6
    /// - Max: 10
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn press_left_tank(&self) -> f32 {
        self.press_left_tank_raw()
    }
    
    /// Get raw value of press_left_tank
    ///
    /// - Start bit: 1
    /// - Signal size: 5 bits
    /// - Factor: 0.25
    /// - Offset: 6
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn press_left_tank_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[1..6].load_le::<u8>();
        
        let factor = 0.25_f32;
        let offset = 6_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of press_left_tank
    #[inline(always)]
    pub fn set_press_left_tank(&mut self, value: f32) -> Result<(), CanError> {
        if value < 6_f32 || 10_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EbsStatus::MESSAGE_ID });
        }
        let factor = 0.25_f32;
        let offset = 6_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[1..6].store_le(value);
        Ok(())
    }
    
    /// press_right_tank
    ///
    /// - Min: 6
    /// - Max: 10
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn press_right_tank(&self) -> f32 {
        self.press_right_tank_raw()
    }
    
    /// Get raw value of press_right_tank
    ///
    /// - Start bit: 6
    /// - Signal size: 5 bits
    /// - Factor: 0.25
    /// - Offset: 6
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn press_right_tank_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[6..11].load_le::<u8>();
        
        let factor = 0.25_f32;
        let offset = 6_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of press_right_tank
    #[inline(always)]
    pub fn set_press_right_tank(&mut self, value: f32) -> Result<(), CanError> {
        if value < 6_f32 || 10_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: EbsStatus::MESSAGE_ID });
        }
        let factor = 0.25_f32;
        let offset = 6_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[6..11].store_le(value);
        Ok(())
    }
    
    /// sanity_left_sensor
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sanity_left_sensor(&self) -> bool {
        self.sanity_left_sensor_raw()
    }
    
    /// Get raw value of sanity_left_sensor
    ///
    /// - Start bit: 11
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sanity_left_sensor_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[11..12].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of sanity_left_sensor
    #[inline(always)]
    pub fn set_sanity_left_sensor(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[11..12].store_le(value);
        Ok(())
    }
    
    /// sanity_right_sensor
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn sanity_right_sensor(&self) -> bool {
        self.sanity_right_sensor_raw()
    }
    
    /// Get raw value of sanity_right_sensor
    ///
    /// - Start bit: 12
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn sanity_right_sensor_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[12..13].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of sanity_right_sensor
    #[inline(always)]
    pub fn set_sanity_right_sensor(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[12..13].store_le(value);
        Ok(())
    }
    
    /// ASB_check
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn asb_check(&self) -> bool {
        self.asb_check_raw()
    }
    
    /// Get raw value of ASB_check
    ///
    /// - Start bit: 13
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn asb_check_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[13..14].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of ASB_check
    #[inline(always)]
    pub fn set_asb_check(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[13..14].store_le(value);
        Ok(())
    }
    
    /// brakes_engaged
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brakes_engaged(&self) -> bool {
        self.brakes_engaged_raw()
    }
    
    /// Get raw value of brakes_engaged
    ///
    /// - Start bit: 14
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brakes_engaged_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[14..15].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of brakes_engaged
    #[inline(always)]
    pub fn set_brakes_engaged(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[14..15].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for EbsStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for EbsStatus {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// CarMission
///
/// - Standard ID: 71 (0x47)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct CarMission {
    raw: [u8; 1],
}

impl CarMission {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x47)});
    
    pub const MISSION_MIN: u8 = 0_u8;
    pub const MISSION_MAX: u8 = 7_u8;
    
    /// Construct new CarMission from values
    pub fn new(mission: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_mission(mission)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Mission
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn mission(&self) -> CarMissionMission {
        let signal = self.raw.view_bits::<Lsb0>()[0..3].load_le::<u8>();
        
        match signal {
            7 => CarMissionMission::DvInspection,
            6 => CarMissionMission::DvEbsTest,
            5 => CarMissionMission::DvTrackdrive,
            4 => CarMissionMission::DvAutocross,
            3 => CarMissionMission::DvSkidpad,
            2 => CarMissionMission::DvAcceleration,
            1 => CarMissionMission::Manualy,
            0 => CarMissionMission::None,
            _ => CarMissionMission::_Other(self.mission_raw()),
        }
    }
    
    /// Get raw value of Mission
    ///
    /// - Start bit: 0
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mission_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..3].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Mission
    #[inline(always)]
    pub fn set_mission(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarMission::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarMission::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..3].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CarMission {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CarMission {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for Mission
#[derive(Clone, Copy, PartialEq)]
pub enum CarMissionMission {
    DvInspection,
    DvEbsTest,
    DvTrackdrive,
    DvAutocross,
    DvSkidpad,
    DvAcceleration,
    Manualy,
    None,
    _Other(u8),
}

impl From<CarMissionMission> for u8 {
    fn from(val: CarMissionMission) -> u8 {
        match val {
            CarMissionMission::DvInspection => 7,
            CarMissionMission::DvEbsTest => 6,
            CarMissionMission::DvTrackdrive => 5,
            CarMissionMission::DvAutocross => 4,
            CarMissionMission::DvSkidpad => 3,
            CarMissionMission::DvAcceleration => 2,
            CarMissionMission::Manualy => 1,
            CarMissionMission::None => 0,
            CarMissionMission::_Other(x) => x,
        }
    }
}


/// PcuFault
///
/// - Standard ID: 81 (0x51)
/// - Size: 1 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct PcuFault {
    raw: [u8; 1],
}

impl PcuFault {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x51)});
    
    
    /// Construct new PcuFault from values
    pub fn new(fault_12v: bool, fault_dv: bool, fault_24v: bool, fault_pumpl: bool, fault_pumpr: bool, fault_fanbattr: bool, fault_fanbattl: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_fault_12v(fault_12v)?;
        res.set_fault_dv(fault_dv)?;
        res.set_fault_24v(fault_24v)?;
        res.set_fault_pumpl(fault_pumpl)?;
        res.set_fault_pumpr(fault_pumpr)?;
        res.set_fault_fanbattr(fault_fanbattr)?;
        res.set_fault_fanbattl(fault_fanbattl)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// fault_12v
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_12v(&self) -> bool {
        self.fault_12v_raw()
    }
    
    /// Get raw value of fault_12v
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_12v_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_12v
    #[inline(always)]
    pub fn set_fault_12v(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// fault_dv
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_dv(&self) -> bool {
        self.fault_dv_raw()
    }
    
    /// Get raw value of fault_dv
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_dv_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_dv
    #[inline(always)]
    pub fn set_fault_dv(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
    /// fault_24v
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_24v(&self) -> bool {
        self.fault_24v_raw()
    }
    
    /// Get raw value of fault_24v
    ///
    /// - Start bit: 2
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_24v_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_24v
    #[inline(always)]
    pub fn set_fault_24v(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
        Ok(())
    }
    
    /// fault_pumpl
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_pumpl(&self) -> bool {
        self.fault_pumpl_raw()
    }
    
    /// Get raw value of fault_pumpl
    ///
    /// - Start bit: 3
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_pumpl_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_pumpl
    #[inline(always)]
    pub fn set_fault_pumpl(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
        Ok(())
    }
    
    /// fault_pumpr
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_pumpr(&self) -> bool {
        self.fault_pumpr_raw()
    }
    
    /// Get raw value of fault_pumpr
    ///
    /// - Start bit: 4
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_pumpr_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[4..5].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_pumpr
    #[inline(always)]
    pub fn set_fault_pumpr(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[4..5].store_le(value);
        Ok(())
    }
    
    /// fault_fanbattr
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_fanbattr(&self) -> bool {
        self.fault_fanbattr_raw()
    }
    
    /// Get raw value of fault_fanbattr
    ///
    /// - Start bit: 5
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_fanbattr_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[5..6].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_fanbattr
    #[inline(always)]
    pub fn set_fault_fanbattr(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[5..6].store_le(value);
        Ok(())
    }
    
    /// fault_fanbattl
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn fault_fanbattl(&self) -> bool {
        self.fault_fanbattl_raw()
    }
    
    /// Get raw value of fault_fanbattl
    ///
    /// - Start bit: 6
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fault_fanbattl_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[6..7].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fault_fanbattl
    #[inline(always)]
    pub fn set_fault_fanbattl(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[6..7].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuFault {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for PcuFault {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Paddle
///
/// - Standard ID: 82 (0x52)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct Paddle {
    raw: [u8; 1],
}

impl Paddle {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x52)});
    
    pub const REGEN_MIN: u8 = 0_u8;
    pub const REGEN_MAX: u8 = 100_u8;
    
    /// Construct new Paddle from values
    pub fn new(regen: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_regen(regen)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// regen
    ///
    /// % of regen paddle travel
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn regen(&self) -> u8 {
        self.regen_raw()
    }
    
    /// Get raw value of regen
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn regen_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of regen
    #[inline(always)]
    pub fn set_regen(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Paddle::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Paddle::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Paddle {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Paddle {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Driver
///
/// - Standard ID: 83 (0x53)
/// - Size: 4 bytes
/// - Transmitter: ATC
#[derive(Clone, Copy)]
pub struct Driver {
    raw: [u8; 4],
}

impl Driver {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x53)});
    
    pub const THROTTLE_MIN: u8 = 0_u8;
    pub const THROTTLE_MAX: u8 = 100_u8;
    pub const BRAKE_MIN: u8 = 0_u8;
    pub const BRAKE_MAX: u8 = 100_u8;
    pub const STEERING_MIN: i16 = -120_i16;
    pub const STEERING_MAX: i16 = 120_i16;
    
    /// Construct new Driver from values
    pub fn new(throttle: u8, brake: u8, steering: i16, no_implausibility: bool, bre_implausibility: bool, pad_implausibility: bool, pot_implausibility: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_throttle(throttle)?;
        res.set_brake(brake)?;
        res.set_steering(steering)?;
        res.set_no_implausibility(no_implausibility)?;
        res.set_bre_implausibility(bre_implausibility)?;
        res.set_pad_implausibility(pad_implausibility)?;
        res.set_pot_implausibility(pot_implausibility)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    
    /// throttle
    ///
    /// % of throttle
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn throttle(&self) -> u8 {
        self.throttle_raw()
    }
    
    /// Get raw value of throttle
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn throttle_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of throttle
    #[inline(always)]
    pub fn set_throttle(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Driver::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Driver::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// brake
    ///
    /// % of brake pedal
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn brake(&self) -> u8 {
        self.brake_raw()
    }
    
    /// Get raw value of brake
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of brake
    #[inline(always)]
    pub fn set_brake(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Driver::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Driver::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// steering
    ///
    /// Steering angle in milli radians
    ///
    /// - Min: -120
    /// - Max: 120
    /// - Unit: "deg"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn steering(&self) -> i16 {
        self.steering_raw()
    }
    
    /// Get raw value of steering
    ///
    /// - Start bit: 16
    /// - Signal size: 12 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn steering_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..28].load_le::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of steering
    #[inline(always)]
    pub fn set_steering(&mut self, value: i16) -> Result<(), CanError> {
        if value < -120_i16 || 120_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Driver::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Driver::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[16..28].store_le(value);
        Ok(())
    }
    
    /// no_implausibility
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn no_implausibility(&self) -> bool {
        self.no_implausibility_raw()
    }
    
    /// Get raw value of no_implausibility
    ///
    /// - Start bit: 28
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn no_implausibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[28..29].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of no_implausibility
    #[inline(always)]
    pub fn set_no_implausibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[28..29].store_le(value);
        Ok(())
    }
    
    /// bre_implausibility
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn bre_implausibility(&self) -> bool {
        self.bre_implausibility_raw()
    }
    
    /// Get raw value of bre_implausibility
    ///
    /// - Start bit: 29
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn bre_implausibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[29..30].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of bre_implausibility
    #[inline(always)]
    pub fn set_bre_implausibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[29..30].store_le(value);
        Ok(())
    }
    
    /// pad_implausibility
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pad_implausibility(&self) -> bool {
        self.pad_implausibility_raw()
    }
    
    /// Get raw value of pad_implausibility
    ///
    /// - Start bit: 30
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pad_implausibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[30..31].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of pad_implausibility
    #[inline(always)]
    pub fn set_pad_implausibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[30..31].store_le(value);
        Ok(())
    }
    
    /// pot_implausibility
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pot_implausibility(&self) -> bool {
        self.pot_implausibility_raw()
    }
    
    /// Get raw value of pot_implausibility
    ///
    /// - Start bit: 31
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pot_implausibility_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[31..32].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of pot_implausibility
    #[inline(always)]
    pub fn set_pot_implausibility(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[31..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Driver {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Driver {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// BmsLv1
///
/// - Standard ID: 84 (0x54)
/// - Size: 7 bytes
/// - Transmitter: BMSHV
#[derive(Clone, Copy)]
pub struct BmsLv1 {
    raw: [u8; 7],
}

impl BmsLv1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x54)});
    
    pub const MAX_VOLT_MIN: f32 = 0_f32;
    pub const MAX_VOLT_MAX: f32 = 0_f32;
    pub const MIN_VOLT_MIN: f32 = 0_f32;
    pub const MIN_VOLT_MAX: f32 = 0_f32;
    pub const AVG_VOLT_MIN: f32 = 0_f32;
    pub const AVG_VOLT_MAX: f32 = 0_f32;
    pub const SOC_MIN: u8 = 0_u8;
    pub const SOC_MAX: u8 = 100_u8;
    
    /// Construct new BmsLv1 from values
    pub fn new(max_volt: f32, min_volt: f32, avg_volt: f32, soc: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_max_volt(max_volt)?;
        res.set_min_volt(min_volt)?;
        res.set_avg_volt(avg_volt)?;
        res.set_soc(soc)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// max_volt
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn max_volt(&self) -> f32 {
        self.max_volt_raw()
    }
    
    /// Get raw value of max_volt
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of max_volt
    #[inline(always)]
    pub fn set_max_volt(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsLv1::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// min_volt
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn min_volt(&self) -> f32 {
        self.min_volt_raw()
    }
    
    /// Get raw value of min_volt
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn min_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of min_volt
    #[inline(always)]
    pub fn set_min_volt(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsLv1::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// avg_volt
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn avg_volt(&self) -> f32 {
        self.avg_volt_raw()
    }
    
    /// Get raw value of avg_volt
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of avg_volt
    #[inline(always)]
    pub fn set_avg_volt(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsLv1::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// soc
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn soc(&self) -> u8 {
        self.soc_raw()
    }
    
    /// Get raw value of soc
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn soc_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of soc
    #[inline(always)]
    pub fn set_soc(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsLv1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsLv1::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLv1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for BmsLv1 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// BmsLv2
///
/// - Standard ID: 85 (0x55)
/// - Size: 7 bytes
/// - Transmitter: BMSHV
#[derive(Clone, Copy)]
pub struct BmsLv2 {
    raw: [u8; 7],
}

impl BmsLv2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x55)});
    
    pub const MAX_TEMP_MIN: u16 = 0_u16;
    pub const MAX_TEMP_MAX: u16 = 0_u16;
    pub const MIN_TEMP_MIN: u16 = 0_u16;
    pub const MIN_TEMP_MAX: u16 = 0_u16;
    pub const AVG_TEMP_MIN: u16 = 0_u16;
    pub const AVG_TEMP_MAX: u16 = 0_u16;
    pub const FAN_SPEED_MIN: u8 = 0_u8;
    pub const FAN_SPEED_MAX: u8 = 100_u8;
    
    /// Construct new BmsLv2 from values
    pub fn new(max_temp: u16, min_temp: u16, avg_temp: u16, fan_speed: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_max_temp(max_temp)?;
        res.set_min_temp(min_temp)?;
        res.set_avg_temp(avg_temp)?;
        res.set_fan_speed(fan_speed)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// max_temp
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn max_temp(&self) -> u16 {
        self.max_temp_raw()
    }
    
    /// Get raw value of max_temp
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of max_temp
    #[inline(always)]
    pub fn set_max_temp(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsLv2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsLv2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// min_temp
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn min_temp(&self) -> u16 {
        self.min_temp_raw()
    }
    
    /// Get raw value of min_temp
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn min_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of min_temp
    #[inline(always)]
    pub fn set_min_temp(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsLv2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsLv2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// avg_temp
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn avg_temp(&self) -> u16 {
        self.avg_temp_raw()
    }
    
    /// Get raw value of avg_temp
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of avg_temp
    #[inline(always)]
    pub fn set_avg_temp(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsLv2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsLv2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// fan_speed
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn fan_speed(&self) -> u8 {
        self.fan_speed_raw()
    }
    
    /// Get raw value of fan_speed
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fan_speed_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of fan_speed
    #[inline(always)]
    pub fn set_fan_speed(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsLv2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsLv2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsLv2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for BmsLv2 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// BmsHv1
///
/// - Standard ID: 87 (0x57)
/// - Size: 7 bytes
/// - Transmitter: BMSHV
#[derive(Clone, Copy)]
pub struct BmsHv1 {
    raw: [u8; 7],
}

impl BmsHv1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x57)});
    
    pub const MAX_VOLT_MIN: f32 = 0_f32;
    pub const MAX_VOLT_MAX: f32 = 0_f32;
    pub const MIN_VOLT_MIN: f32 = 0_f32;
    pub const MIN_VOLT_MAX: f32 = 0_f32;
    pub const AVG_VOLT_MIN: f32 = 0_f32;
    pub const AVG_VOLT_MAX: f32 = 0_f32;
    pub const SOC_MIN: u8 = 0_u8;
    pub const SOC_MAX: u8 = 100_u8;
    
    /// Construct new BmsHv1 from values
    pub fn new(max_volt: f32, min_volt: f32, avg_volt: f32, soc: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_max_volt(max_volt)?;
        res.set_min_volt(min_volt)?;
        res.set_avg_volt(avg_volt)?;
        res.set_soc(soc)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// max_volt
    ///
    /// Maximum cell voltage in mv
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn max_volt(&self) -> f32 {
        self.max_volt_raw()
    }
    
    /// Get raw value of max_volt
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of max_volt
    #[inline(always)]
    pub fn set_max_volt(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsHv1::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// min_volt
    ///
    /// Minimum cell voltage in mv
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn min_volt(&self) -> f32 {
        self.min_volt_raw()
    }
    
    /// Get raw value of min_volt
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn min_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of min_volt
    #[inline(always)]
    pub fn set_min_volt(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsHv1::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// avg_volt
    ///
    /// Average cell voltage in mv
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mV"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn avg_volt(&self) -> f32 {
        self.avg_volt_raw()
    }
    
    /// Get raw value of avg_volt
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of avg_volt
    #[inline(always)]
    pub fn set_avg_volt(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsHv1::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// soc
    ///
    /// HV battery SOC in %
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn soc(&self) -> u8 {
        self.soc_raw()
    }
    
    /// Get raw value of soc
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn soc_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of soc
    #[inline(always)]
    pub fn set_soc(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsHv1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsHv1::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsHv1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for BmsHv1 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// BmsHv2
///
/// - Standard ID: 88 (0x58)
/// - Size: 7 bytes
/// - Transmitter: BMSHV
#[derive(Clone, Copy)]
pub struct BmsHv2 {
    raw: [u8; 7],
}

impl BmsHv2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x58)});
    
    pub const MAX_TEMP_MIN: u16 = 0_u16;
    pub const MAX_TEMP_MAX: u16 = 0_u16;
    pub const MIN_TEMP_MIN: u16 = 0_u16;
    pub const MIN_TEMP_MAX: u16 = 0_u16;
    pub const AVG_TEMP_MIN: u16 = 0_u16;
    pub const AVG_TEMP_MAX: u16 = 0_u16;
    pub const FAN_SPEED_MIN: u8 = 0_u8;
    pub const FAN_SPEED_MAX: u8 = 100_u8;
    
    /// Construct new BmsHv2 from values
    pub fn new(max_temp: u16, min_temp: u16, avg_temp: u16, fan_speed: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_max_temp(max_temp)?;
        res.set_min_temp(min_temp)?;
        res.set_avg_temp(avg_temp)?;
        res.set_fan_speed(fan_speed)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// max_temp
    ///
    /// Maximum cell temperature in celsius
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn max_temp(&self) -> u16 {
        self.max_temp_raw()
    }
    
    /// Get raw value of max_temp
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of max_temp
    #[inline(always)]
    pub fn set_max_temp(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsHv2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsHv2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// min_temp
    ///
    /// Minimum cell temperature in celsius
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn min_temp(&self) -> u16 {
        self.min_temp_raw()
    }
    
    /// Get raw value of min_temp
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn min_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of min_temp
    #[inline(always)]
    pub fn set_min_temp(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsHv2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsHv2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// avg_temp
    ///
    /// Average cell temperature in celsius
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn avg_temp(&self) -> u16 {
        self.avg_temp_raw()
    }
    
    /// Get raw value of avg_temp
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_temp_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of avg_temp
    #[inline(always)]
    pub fn set_avg_temp(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsHv2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsHv2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// fan_speed
    ///
    /// % of HV battery fan speed
    ///
    /// - Min: 0
    /// - Max: 100
    /// - Unit: "%"
    /// - Receivers: VCU, SW
    #[inline(always)]
    pub fn fan_speed(&self) -> u8 {
        self.fan_speed_raw()
    }
    
    /// Get raw value of fan_speed
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fan_speed_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of fan_speed
    #[inline(always)]
    pub fn set_fan_speed(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 100_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: BmsHv2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: BmsHv2::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for BmsHv2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for BmsHv2 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Imu1
///
/// - Standard ID: 96 (0x60)
/// - Size: 8 bytes
/// - Transmitter: SMU
#[derive(Clone, Copy)]
pub struct Imu1 {
    raw: [u8; 8],
}

impl Imu1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x60)});
    
    pub const ACC_X_MIN: i32 = 0_i32;
    pub const ACC_X_MAX: i32 = 0_i32;
    pub const ACC_Y_MIN: i32 = 0_i32;
    pub const ACC_Y_MAX: i32 = 0_i32;
    
    /// Construct new Imu1 from values
    pub fn new(acc_x: i32, acc_y: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_acc_x(acc_x)?;
        res.set_acc_y(acc_y)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// acc_x
    ///
    /// Acceleration on x axis in m/s^2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn acc_x(&self) -> i32 {
        self.acc_x_raw()
    }
    
    /// Get raw value of acc_x
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn acc_x_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of acc_x
    #[inline(always)]
    pub fn set_acc_x(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Imu1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Imu1::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// acc_y
    ///
    /// Acceleration on y axis in m/s^2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn acc_y(&self) -> i32 {
        self.acc_y_raw()
    }
    
    /// Get raw value of acc_y
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn acc_y_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of acc_y
    #[inline(always)]
    pub fn set_acc_y(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Imu1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Imu1::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Imu1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Imu1 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Imu2
///
/// - Standard ID: 97 (0x61)
/// - Size: 8 bytes
/// - Transmitter: SMU
#[derive(Clone, Copy)]
pub struct Imu2 {
    raw: [u8; 8],
}

impl Imu2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x61)});
    
    pub const ACC_Z_MIN: i32 = 0_i32;
    pub const ACC_Z_MAX: i32 = 0_i32;
    pub const OMEGA_X_MIN: i32 = 0_i32;
    pub const OMEGA_X_MAX: i32 = 0_i32;
    
    /// Construct new Imu2 from values
    pub fn new(acc_z: i32, omega_x: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_acc_z(acc_z)?;
        res.set_omega_x(omega_x)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// acc_z
    ///
    /// Acceleration on z axis in m/s^2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "m/s^2"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn acc_z(&self) -> i32 {
        self.acc_z_raw()
    }
    
    /// Get raw value of acc_z
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn acc_z_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of acc_z
    #[inline(always)]
    pub fn set_acc_z(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Imu2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Imu2::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// omega_x
    ///
    /// Angular speed on x axis in rad/s
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rad/s"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn omega_x(&self) -> i32 {
        self.omega_x_raw()
    }
    
    /// Get raw value of omega_x
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn omega_x_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of omega_x
    #[inline(always)]
    pub fn set_omega_x(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Imu2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Imu2::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Imu2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Imu2 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Imu3
///
/// - Standard ID: 98 (0x62)
/// - Size: 8 bytes
/// - Transmitter: SMU
#[derive(Clone, Copy)]
pub struct Imu3 {
    raw: [u8; 8],
}

impl Imu3 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x62)});
    
    pub const OMEGA_Y_MIN: i32 = 0_i32;
    pub const OMEGA_Y_MAX: i32 = 0_i32;
    pub const OMEGA_Z_MIN: i32 = 0_i32;
    pub const OMEGA_Z_MAX: i32 = 0_i32;
    
    /// Construct new Imu3 from values
    pub fn new(omega_y: i32, omega_z: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_omega_y(omega_y)?;
        res.set_omega_z(omega_z)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// omega_y
    ///
    /// Angular speed on x axis in rad/s
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rad/s"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn omega_y(&self) -> i32 {
        self.omega_y_raw()
    }
    
    /// Get raw value of omega_y
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn omega_y_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of omega_y
    #[inline(always)]
    pub fn set_omega_y(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Imu3::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Imu3::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// omega_z
    ///
    /// Angular speed on x axis in rad/s
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "rad/s"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn omega_z(&self) -> i32 {
        self.omega_z_raw()
    }
    
    /// Get raw value of omega_z
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn omega_z_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<i32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of omega_z
    #[inline(always)]
    pub fn set_omega_z(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Imu3::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Imu3::MESSAGE_ID })?;
        let value = (value / factor) as i32;
        
        let value = u32::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Imu3 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Imu3 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// IMUCalib
///
/// - Standard ID: 99 (0x63)
/// - Size: 1 bytes
/// - Transmitter: SMU
///
/// RESERVER FOR IMU mask - DO NOT USE
#[derive(Clone, Copy)]
pub struct ImuCalib {
    raw: [u8; 1],
}

impl ImuCalib {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x63)});
    
    
    /// Construct new IMUCalib from values
    pub fn new(start_imu_calibration: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_start_imu_calibration(start_imu_calibration)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// start_imu_calibration
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn start_imu_calibration(&self) -> bool {
        self.start_imu_calibration_raw()
    }
    
    /// Get raw value of start_imu_calibration
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn start_imu_calibration_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of start_imu_calibration
    #[inline(always)]
    pub fn set_start_imu_calibration(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ImuCalib {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ImuCalib {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Map
///
/// - Standard ID: 100 (0x64)
/// - Size: 2 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct Map {
    raw: [u8; 2],
}

impl Map {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x64)});
    
    pub const POWER_MIN: u8 = 1_u8;
    pub const POWER_MAX: u8 = 12_u8;
    pub const REGEN_MIN: u8 = 1_u8;
    pub const REGEN_MAX: u8 = 12_u8;
    pub const TORQUE_REP_MIN: u8 = 0_u8;
    pub const TORQUE_REP_MAX: u8 = 12_u8;
    
    /// Construct new Map from values
    pub fn new(power: u8, regen: u8, torque_rep: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_power(power)?;
        res.set_regen(regen)?;
        res.set_torque_rep(torque_rep)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// power
    ///
    /// Map selected number
    ///
    /// - Min: 1
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn power(&self) -> u8 {
        self.power_raw()
    }
    
    /// Get raw value of power
    ///
    /// - Start bit: 0
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn power_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..4].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of power
    #[inline(always)]
    pub fn set_power(&mut self, value: u8) -> Result<(), CanError> {
        if value < 1_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Map::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Map::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..4].store_le(value);
        Ok(())
    }
    
    /// regen
    ///
    /// Map selected for regen braking
    ///
    /// - Min: 1
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn regen(&self) -> u8 {
        self.regen_raw()
    }
    
    /// Get raw value of regen
    ///
    /// - Start bit: 4
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn regen_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[4..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of regen
    #[inline(always)]
    pub fn set_regen(&mut self, value: u8) -> Result<(), CanError> {
        if value < 1_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Map::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Map::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[4..8].store_le(value);
        Ok(())
    }
    
    /// torque_rep
    ///
    /// Map selected for torque repeartition
    ///
    /// - Min: 0
    /// - Max: 12
    /// - Unit: "map"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn torque_rep(&self) -> u8 {
        self.torque_rep_raw()
    }
    
    /// Get raw value of torque_rep
    ///
    /// - Start bit: 8
    /// - Signal size: 4 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn torque_rep_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..12].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of torque_rep
    #[inline(always)]
    pub fn set_torque_rep(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 12_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Map::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Map::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..12].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Map {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Map {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// CarStatus
///
/// - Standard ID: 101 (0x65)
/// - Size: 4 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CarStatus {
    raw: [u8; 4],
}

impl CarStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x65)});
    
    pub const RUNNING_STATUS_MIN: u8 = 0_u8;
    pub const RUNNING_STATUS_MAX: u8 = 3_u8;
    pub const SPEED_MIN: u8 = 0_u8;
    pub const SPEED_MAX: u8 = 0_u8;
    pub const BRAKE_FRONT_PRESS_MIN: f32 = 0_f32;
    pub const BRAKE_FRONT_PRESS_MAX: f32 = 60_f32;
    pub const BRAKE_REAR_PRESS_MIN: f32 = 0_f32;
    pub const BRAKE_REAR_PRESS_MAX: f32 = 60_f32;
    
    /// Construct new CarStatus from values
    pub fn new(hv: bool, air1: bool, air2: bool, running_status: u8, speed: u8, brake_front_press: f32, brake_rear_press: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_hv(hv)?;
        res.set_air1(air1)?;
        res.set_air2(air2)?;
        res.set_running_status(running_status)?;
        res.set_speed(speed)?;
        res.set_brake_front_press(brake_front_press)?;
        res.set_brake_rear_press(brake_rear_press)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 4] {
        &self.raw
    }
    
    /// HV
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn hv(&self) -> bool {
        self.hv_raw()
    }
    
    /// Get raw value of HV
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn hv_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of HV
    #[inline(always)]
    pub fn set_hv(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// AIR1
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "closed"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn air1(&self) -> bool {
        self.air1_raw()
    }
    
    /// Get raw value of AIR1
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn air1_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of AIR1
    #[inline(always)]
    pub fn set_air1(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
    /// AIR2
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "closed"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn air2(&self) -> bool {
        self.air2_raw()
    }
    
    /// Get raw value of AIR2
    ///
    /// - Start bit: 2
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn air2_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of AIR2
    #[inline(always)]
    pub fn set_air2(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
        Ok(())
    }
    
    /// RunningStatus
    ///
    /// - Min: 0
    /// - Max: 3
    /// - Unit: "phase"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn running_status(&self) -> CarStatusRunningStatus {
        let signal = self.raw.view_bits::<Lsb0>()[3..5].load_le::<u8>();
        
        match signal {
            3 => CarStatusRunningStatus::Running,
            2 => CarStatusRunningStatus::TsReady,
            1 => CarStatusRunningStatus::PrechargeStarted,
            0 => CarStatusRunningStatus::SystemOff,
            _ => CarStatusRunningStatus::_Other(self.running_status_raw()),
        }
    }
    
    /// Get raw value of RunningStatus
    ///
    /// - Start bit: 3
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn running_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[3..5].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of RunningStatus
    #[inline(always)]
    pub fn set_running_status(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 3_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[3..5].store_le(value);
        Ok(())
    }
    
    /// speed
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "km/h"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn speed(&self) -> u8 {
        self.speed_raw()
    }
    
    /// Get raw value of speed
    ///
    /// - Start bit: 5
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn speed_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[5..13].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of speed
    #[inline(always)]
    pub fn set_speed(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[5..13].store_le(value);
        Ok(())
    }
    
    /// brake_front_press
    ///
    /// - Min: 0
    /// - Max: 60
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_front_press(&self) -> f32 {
        self.brake_front_press_raw()
    }
    
    /// Get raw value of brake_front_press
    ///
    /// - Start bit: 13
    /// - Signal size: 8 bits
    /// - Factor: 0.25
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_front_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[13..21].load_le::<u8>();
        
        let factor = 0.25_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_front_press
    #[inline(always)]
    pub fn set_brake_front_press(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarStatus::MESSAGE_ID });
        }
        let factor = 0.25_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[13..21].store_le(value);
        Ok(())
    }
    
    /// brake_rear_press
    ///
    /// - Min: 0
    /// - Max: 60
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_rear_press(&self) -> f32 {
        self.brake_rear_press_raw()
    }
    
    /// Get raw value of brake_rear_press
    ///
    /// - Start bit: 21
    /// - Signal size: 8 bits
    /// - Factor: 0.25
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_rear_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[21..29].load_le::<u8>();
        
        let factor = 0.25_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_rear_press
    #[inline(always)]
    pub fn set_brake_rear_press(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 60_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarStatus::MESSAGE_ID });
        }
        let factor = 0.25_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[21..29].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CarStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CarStatus {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for RunningStatus
#[derive(Clone, Copy, PartialEq)]
pub enum CarStatusRunningStatus {
    Running,
    TsReady,
    PrechargeStarted,
    SystemOff,
    _Other(u8),
}

impl From<CarStatusRunningStatus> for u8 {
    fn from(val: CarStatusRunningStatus) -> u8 {
        match val {
            CarStatusRunningStatus::Running => 3,
            CarStatusRunningStatus::TsReady => 2,
            CarStatusRunningStatus::PrechargeStarted => 1,
            CarStatusRunningStatus::SystemOff => 0,
            CarStatusRunningStatus::_Other(x) => x,
        }
    }
}


/// CarSettings
///
/// - Standard ID: 102 (0x66)
/// - Size: 8 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CarSettings {
    raw: [u8; 8],
}

impl CarSettings {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x66)});
    
    pub const MAX_REGEN_CURRENT_MIN: u8 = 0_u8;
    pub const MAX_REGEN_CURRENT_MAX: u8 = 150_u8;
    pub const PWR_LIMIT_MIN: u8 = 0_u8;
    pub const PWR_LIMIT_MAX: u8 = 80_u8;
    pub const SPEED_LIM_MIN: u8 = 0_u8;
    pub const SPEED_LIM_MAX: u8 = 0_u8;
    pub const MAX_POS_TRQ_MIN: u8 = 0_u8;
    pub const MAX_POS_TRQ_MAX: u8 = 0_u8;
    pub const MAX_NEG_TRQ_MIN: i8 = 0_i8;
    pub const MAX_NEG_TRQ_MAX: i8 = 0_i8;
    pub const FRONT_MOTOR_REPARTITION_MIN: u8 = 0_u8;
    pub const FRONT_MOTOR_REPARTITION_MAX: u8 = 0_u8;
    pub const REAR_MOTOR_REPARTITION_MIN: u8 = 0_u8;
    pub const REAR_MOTOR_REPARTITION_MAX: u8 = 0_u8;
    
    /// Construct new CarSettings from values
    pub fn new(max_regen_current: u8, pwr_limit: u8, speed_lim: u8, max_pos_trq: u8, max_neg_trq: i8, front_motor_repartition: u8, rear_motor_repartition: u8, torque_vectoring: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_max_regen_current(max_regen_current)?;
        res.set_pwr_limit(pwr_limit)?;
        res.set_speed_lim(speed_lim)?;
        res.set_max_pos_trq(max_pos_trq)?;
        res.set_max_neg_trq(max_neg_trq)?;
        res.set_front_motor_repartition(front_motor_repartition)?;
        res.set_rear_motor_repartition(rear_motor_repartition)?;
        res.set_torque_vectoring(torque_vectoring)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// max_regen_current
    ///
    /// Maximum Regen Current
    ///
    /// - Min: 0
    /// - Max: 150
    /// - Unit: "A"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_regen_current(&self) -> u8 {
        self.max_regen_current_raw()
    }
    
    /// Get raw value of max_regen_current
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_regen_current_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of max_regen_current
    #[inline(always)]
    pub fn set_max_regen_current(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 150_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// pwr_limit
    ///
    /// Maximum power limit
    ///
    /// - Min: 0
    /// - Max: 80
    /// - Unit: "kW"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn pwr_limit(&self) -> u8 {
        self.pwr_limit_raw()
    }
    
    /// Get raw value of pwr_limit
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pwr_limit_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of pwr_limit
    #[inline(always)]
    pub fn set_pwr_limit(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 80_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
    /// speed_lim
    ///
    /// Maximum Speed Limit
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "krpm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn speed_lim(&self) -> u8 {
        self.speed_lim_raw()
    }
    
    /// Get raw value of speed_lim
    ///
    /// - Start bit: 16
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn speed_lim_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[16..24].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of speed_lim
    #[inline(always)]
    pub fn set_speed_lim(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[16..24].store_le(value);
        Ok(())
    }
    
    /// max_pos_trq
    ///
    /// Maximum Positive Torque
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_pos_trq(&self) -> u8 {
        self.max_pos_trq_raw()
    }
    
    /// Get raw value of max_pos_trq
    ///
    /// - Start bit: 24
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_pos_trq_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[24..32].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of max_pos_trq
    #[inline(always)]
    pub fn set_max_pos_trq(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[24..32].store_le(value);
        Ok(())
    }
    
    /// max_neg_trq
    ///
    /// Maximum Negative Torque
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Nm"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn max_neg_trq(&self) -> i8 {
        self.max_neg_trq_raw()
    }
    
    /// Get raw value of max_neg_trq
    ///
    /// - Start bit: 32
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn max_neg_trq_raw(&self) -> i8 {
        let signal = self.raw.view_bits::<Lsb0>()[32..40].load_le::<i8>();
        
        let factor = 1;
        let signal = signal as i8;
        i8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of max_neg_trq
    #[inline(always)]
    pub fn set_max_neg_trq(&mut self, value: i8) -> Result<(), CanError> {
        if value < 0_i8 || 0_i8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID })?;
        let value = (value / factor) as i8;
        
        let value = u8::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Lsb0>()[32..40].store_le(value);
        Ok(())
    }
    
    /// front_motor_repartition
    ///
    /// Torque Repartition Front
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn front_motor_repartition(&self) -> u8 {
        self.front_motor_repartition_raw()
    }
    
    /// Get raw value of front_motor_repartition
    ///
    /// - Start bit: 40
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn front_motor_repartition_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[40..48].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of front_motor_repartition
    #[inline(always)]
    pub fn set_front_motor_repartition(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[40..48].store_le(value);
        Ok(())
    }
    
    /// rear_motor_repartition
    ///
    /// Torque Repartition Rear
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "%"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn rear_motor_repartition(&self) -> u8 {
        self.rear_motor_repartition_raw()
    }
    
    /// Get raw value of rear_motor_repartition
    ///
    /// - Start bit: 48
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rear_motor_repartition_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[48..56].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of rear_motor_repartition
    #[inline(always)]
    pub fn set_rear_motor_repartition(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 0_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarSettings::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[48..56].store_le(value);
        Ok(())
    }
    
    /// torque_vectoring
    ///
    /// Torque Vectoring enabled
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn torque_vectoring(&self) -> bool {
        self.torque_vectoring_raw()
    }
    
    /// Get raw value of torque_vectoring
    ///
    /// - Start bit: 56
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn torque_vectoring_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[56..57].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of torque_vectoring
    #[inline(always)]
    pub fn set_torque_vectoring(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[56..57].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CarSettings {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CarSettings {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// CheckASBReq
///
/// - Standard ID: 104 (0x68)
/// - Size: 1 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CheckAsbReq {
    raw: [u8; 1],
}

impl CheckAsbReq {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x68)});
    
    
    /// Construct new CheckASBReq from values
    pub fn new(req: bool, req_ack: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_req(req)?;
        res.set_req_ack(req_ack)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// req
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn req(&self) -> bool {
        self.req_raw()
    }
    
    /// Get raw value of req
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn req_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of req
    #[inline(always)]
    pub fn set_req(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// reqAck
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn req_ack(&self) -> bool {
        self.req_ack_raw()
    }
    
    /// Get raw value of reqAck
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn req_ack_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of reqAck
    #[inline(always)]
    pub fn set_req_ack(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CheckAsbReq {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CheckAsbReq {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// EbsBrakeReq
///
/// - Standard ID: 105 (0x69)
/// - Size: 1 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct EbsBrakeReq {
    raw: [u8; 1],
}

impl EbsBrakeReq {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x69)});
    
    
    /// Construct new EbsBrakeReq from values
    pub fn new(req: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_req(req)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// req
    ///
    /// - Min: 0
    /// - Max: 2
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn req(&self) -> bool {
        self.req_raw()
    }
    
    /// Get raw value of req
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn req_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of req
    #[inline(always)]
    pub fn set_req(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for EbsBrakeReq {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for EbsBrakeReq {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// ResStatus
///
/// - Standard ID: 106 (0x6a)
/// - Size: 1 bytes
#[derive(Clone, Copy)]
pub struct ResStatus {
    raw: [u8; 1],
}

impl ResStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x6a)});
    
    
    /// Construct new ResStatus from values
    pub fn new(data: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_data(data)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// data
    ///
    /// - Min: 0
    /// - Max: 2
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn data(&self) -> bool {
        self.data_raw()
    }
    
    /// Get raw value of data
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn data_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of data
    #[inline(always)]
    pub fn set_data(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ResStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ResStatus {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// LapStart
///
/// - Standard ID: 112 (0x70)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct LapStart {
    raw: [u8; 1],
}

impl LapStart {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x70)});
    
    pub const START_MIN: u8 = 0_u8;
    pub const START_MAX: u8 = 1_u8;
    
    /// Construct new LapStart from values
    pub fn new(start: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_start(start)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// start
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "start"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn start(&self) -> u8 {
        self.start_raw()
    }
    
    /// Get raw value of start
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn start_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of start
    #[inline(always)]
    pub fn set_start(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: LapStart::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: LapStart::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for LapStart {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for LapStart {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// CarMissionStatus
///
/// - Standard ID: 113 (0x71)
/// - Size: 1 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct CarMissionStatus {
    raw: [u8; 1],
}

impl CarMissionStatus {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x71)});
    
    pub const MISSION_MIN: u8 = 0_u8;
    pub const MISSION_MAX: u8 = 7_u8;
    pub const MISSION_STATUS_MIN: u8 = 0_u8;
    pub const MISSION_STATUS_MAX: u8 = 2_u8;
    pub const AS_STATUS_MIN: u8 = 0_u8;
    pub const AS_STATUS_MAX: u8 = 7_u8;
    
    /// Construct new CarMissionStatus from values
    pub fn new(mission: u8, mission_status: u8, as_status: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_mission(mission)?;
        res.set_mission_status(mission_status)?;
        res.set_as_status(as_status)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Mission
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: SW
    #[inline(always)]
    pub fn mission(&self) -> CarMissionStatusMission {
        let signal = self.raw.view_bits::<Lsb0>()[0..3].load_le::<u8>();
        
        match signal {
            7 => CarMissionStatusMission::DvInspection,
            6 => CarMissionStatusMission::DvEbsTest,
            5 => CarMissionStatusMission::DvTrackdrive,
            4 => CarMissionStatusMission::DvAutocross,
            3 => CarMissionStatusMission::DvSkidpad,
            2 => CarMissionStatusMission::DvAcceleration,
            1 => CarMissionStatusMission::Manualy,
            0 => CarMissionStatusMission::None,
            _ => CarMissionStatusMission::_Other(self.mission_raw()),
        }
    }
    
    /// Get raw value of Mission
    ///
    /// - Start bit: 0
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mission_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..3].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of Mission
    #[inline(always)]
    pub fn set_mission(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarMissionStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarMissionStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..3].store_le(value);
        Ok(())
    }
    
    /// MissionStatus
    ///
    /// - Min: 0
    /// - Max: 2
    /// - Unit: ""
    /// - Receivers: SW
    #[inline(always)]
    pub fn mission_status(&self) -> CarMissionStatusMissionStatus {
        let signal = self.raw.view_bits::<Lsb0>()[3..5].load_le::<u8>();
        
        match signal {
            2 => CarMissionStatusMissionStatus::MissionFinished,
            1 => CarMissionStatusMissionStatus::MissionRunning,
            0 => CarMissionStatusMissionStatus::MissionNotRunning,
            _ => CarMissionStatusMissionStatus::_Other(self.mission_status_raw()),
        }
    }
    
    /// Get raw value of MissionStatus
    ///
    /// - Start bit: 3
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mission_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[3..5].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of MissionStatus
    #[inline(always)]
    pub fn set_mission_status(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 2_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarMissionStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarMissionStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[3..5].store_le(value);
        Ok(())
    }
    
    /// AsStatus
    ///
    /// - Min: 0
    /// - Max: 7
    /// - Unit: ""
    /// - Receivers: SW
    #[inline(always)]
    pub fn as_status(&self) -> CarMissionStatusAsStatus {
        let signal = self.raw.view_bits::<Lsb0>()[5..8].load_le::<u8>();
        
        match signal {
            5 => CarMissionStatusAsStatus::Finish,
            4 => CarMissionStatusAsStatus::EmergencyBrake,
            3 => CarMissionStatusAsStatus::Driving,
            2 => CarMissionStatusAsStatus::Ready,
            1 => CarMissionStatusAsStatus::Off,
            _ => CarMissionStatusAsStatus::_Other(self.as_status_raw()),
        }
    }
    
    /// Get raw value of AsStatus
    ///
    /// - Start bit: 5
    /// - Signal size: 3 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn as_status_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[5..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of AsStatus
    #[inline(always)]
    pub fn set_as_status(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 7_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CarMissionStatus::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CarMissionStatus::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[5..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CarMissionStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CarMissionStatus {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for Mission
#[derive(Clone, Copy, PartialEq)]
pub enum CarMissionStatusMission {
    DvInspection,
    DvEbsTest,
    DvTrackdrive,
    DvAutocross,
    DvSkidpad,
    DvAcceleration,
    Manualy,
    None,
    _Other(u8),
}

impl From<CarMissionStatusMission> for u8 {
    fn from(val: CarMissionStatusMission) -> u8 {
        match val {
            CarMissionStatusMission::DvInspection => 7,
            CarMissionStatusMission::DvEbsTest => 6,
            CarMissionStatusMission::DvTrackdrive => 5,
            CarMissionStatusMission::DvAutocross => 4,
            CarMissionStatusMission::DvSkidpad => 3,
            CarMissionStatusMission::DvAcceleration => 2,
            CarMissionStatusMission::Manualy => 1,
            CarMissionStatusMission::None => 0,
            CarMissionStatusMission::_Other(x) => x,
        }
    }
}

/// Defined values for MissionStatus
#[derive(Clone, Copy, PartialEq)]
pub enum CarMissionStatusMissionStatus {
    MissionFinished,
    MissionRunning,
    MissionNotRunning,
    _Other(u8),
}

impl From<CarMissionStatusMissionStatus> for u8 {
    fn from(val: CarMissionStatusMissionStatus) -> u8 {
        match val {
            CarMissionStatusMissionStatus::MissionFinished => 2,
            CarMissionStatusMissionStatus::MissionRunning => 1,
            CarMissionStatusMissionStatus::MissionNotRunning => 0,
            CarMissionStatusMissionStatus::_Other(x) => x,
        }
    }
}

/// Defined values for AsStatus
#[derive(Clone, Copy, PartialEq)]
pub enum CarMissionStatusAsStatus {
    Finish,
    EmergencyBrake,
    Driving,
    Ready,
    Off,
    _Other(u8),
}

impl From<CarMissionStatusAsStatus> for u8 {
    fn from(val: CarMissionStatusAsStatus) -> u8 {
        match val {
            CarMissionStatusAsStatus::Finish => 5,
            CarMissionStatusAsStatus::EmergencyBrake => 4,
            CarMissionStatusAsStatus::Driving => 3,
            CarMissionStatusAsStatus::Ready => 2,
            CarMissionStatusAsStatus::Off => 1,
            CarMissionStatusAsStatus::_Other(x) => x,
        }
    }
}


/// Temp1
///
/// - Standard ID: 256 (0x100)
/// - Size: 8 bytes
/// - Transmitter: SMU
#[derive(Clone, Copy)]
pub struct Temp1 {
    raw: [u8; 8],
}

impl Temp1 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x100)});
    
    pub const TEMP_MOTOR_POST_L_MIN: u16 = 0_u16;
    pub const TEMP_MOTOR_POST_L_MAX: u16 = 0_u16;
    pub const TEMP_MOTOR_PRE_L_MIN: u16 = 0_u16;
    pub const TEMP_MOTOR_PRE_L_MAX: u16 = 0_u16;
    pub const TEMP_MOTOR_PRE_R_MIN: u16 = 0_u16;
    pub const TEMP_MOTOR_PRE_R_MAX: u16 = 0_u16;
    pub const TEMP_COLDPLATE_PRE_R_MIN: u16 = 0_u16;
    pub const TEMP_COLDPLATE_PRE_R_MAX: u16 = 0_u16;
    
    /// Construct new Temp1 from values
    pub fn new(temp_motor_post_l: u16, temp_motor_pre_l: u16, temp_motor_pre_r: u16, temp_coldplate_pre_r: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_temp_motor_post_l(temp_motor_post_l)?;
        res.set_temp_motor_pre_l(temp_motor_pre_l)?;
        res.set_temp_motor_pre_r(temp_motor_pre_r)?;
        res.set_temp_coldplate_pre_r(temp_coldplate_pre_r)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// temp_motor_post_L
    ///
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_motor_post_l(&self) -> u16 {
        self.temp_motor_post_l_raw()
    }
    
    /// Get raw value of temp_motor_post_L
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_motor_post_l_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_motor_post_L
    #[inline(always)]
    pub fn set_temp_motor_post_l(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Temp1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Temp1::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// temp_motor_pre_L
    ///
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_motor_pre_l(&self) -> u16 {
        self.temp_motor_pre_l_raw()
    }
    
    /// Get raw value of temp_motor_pre_L
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_motor_pre_l_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_motor_pre_L
    #[inline(always)]
    pub fn set_temp_motor_pre_l(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Temp1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Temp1::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// temp_motor_pre_R
    ///
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_motor_pre_r(&self) -> u16 {
        self.temp_motor_pre_r_raw()
    }
    
    /// Get raw value of temp_motor_pre_R
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_motor_pre_r_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_motor_pre_R
    #[inline(always)]
    pub fn set_temp_motor_pre_r(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Temp1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Temp1::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// temp_coldplate_pre_R
    ///
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_coldplate_pre_r(&self) -> u16 {
        self.temp_coldplate_pre_r_raw()
    }
    
    /// Get raw value of temp_coldplate_pre_R
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_coldplate_pre_r_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_coldplate_pre_R
    #[inline(always)]
    pub fn set_temp_coldplate_pre_r(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Temp1::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Temp1::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Temp1 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Temp1 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Temp2
///
/// - Standard ID: 257 (0x101)
/// - Size: 8 bytes
/// - Transmitter: SMU
#[derive(Clone, Copy)]
pub struct Temp2 {
    raw: [u8; 8],
}

impl Temp2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x101)});
    
    pub const TEMP_COLD_PRE_L_MIN: u16 = 0_u16;
    pub const TEMP_COLD_PRE_L_MAX: u16 = 0_u16;
    pub const TEMP_COLD_POST_R_MIN: u16 = 0_u16;
    pub const TEMP_COLD_POST_R_MAX: u16 = 0_u16;
    pub const TEMP_COLD_POST_L_MIN: u16 = 0_u16;
    pub const TEMP_COLD_POST_L_MAX: u16 = 0_u16;
    pub const TEMP_MOT_POST_R_MIN: u16 = 0_u16;
    pub const TEMP_MOT_POST_R_MAX: u16 = 0_u16;
    
    /// Construct new Temp2 from values
    pub fn new(temp_cold_pre_l: u16, temp_cold_post_r: u16, temp_cold_post_l: u16, temp_mot_post_r: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_temp_cold_pre_l(temp_cold_pre_l)?;
        res.set_temp_cold_post_r(temp_cold_post_r)?;
        res.set_temp_cold_post_l(temp_cold_post_l)?;
        res.set_temp_mot_post_r(temp_mot_post_r)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// temp_cold_pre_L
    ///
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_cold_pre_l(&self) -> u16 {
        self.temp_cold_pre_l_raw()
    }
    
    /// Get raw value of temp_cold_pre_L
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_cold_pre_l_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_cold_pre_L
    #[inline(always)]
    pub fn set_temp_cold_pre_l(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Temp2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Temp2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// temp_cold_post_R
    ///
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_cold_post_r(&self) -> u16 {
        self.temp_cold_post_r_raw()
    }
    
    /// Get raw value of temp_cold_post_R
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_cold_post_r_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_cold_post_R
    #[inline(always)]
    pub fn set_temp_cold_post_r(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Temp2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Temp2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// temp_cold_post_L
    ///
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_cold_post_l(&self) -> u16 {
        self.temp_cold_post_l_raw()
    }
    
    /// Get raw value of temp_cold_post_L
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_cold_post_l_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_cold_post_L
    #[inline(always)]
    pub fn set_temp_cold_post_l(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Temp2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Temp2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// temp_mot_post_R
    ///
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn temp_mot_post_r(&self) -> u16 {
        self.temp_mot_post_r_raw()
    }
    
    /// Get raw value of temp_mot_post_R
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_mot_post_r_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_mot_post_R
    #[inline(always)]
    pub fn set_temp_mot_post_r(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Temp2::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Temp2::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Temp2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Temp2 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// SuspRear
///
/// - Standard ID: 258 (0x102)
/// - Size: 3 bytes
/// - Transmitter: SMU
#[derive(Clone, Copy)]
pub struct SuspRear {
    raw: [u8; 3],
}

impl SuspRear {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x102)});
    
    pub const SUSP_RR_MIN: f32 = 0_f32;
    pub const SUSP_RR_MAX: f32 = 0_f32;
    pub const SUSP_RL_MIN: f32 = 0_f32;
    pub const SUSP_RL_MAX: f32 = 0_f32;
    
    /// Construct new SuspRear from values
    pub fn new(susp_rr: f32, susp_rl: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_susp_rr(susp_rr)?;
        res.set_susp_rl(susp_rl)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// susp_rr
    ///
    /// RR suspension travel in mm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mm"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn susp_rr(&self) -> f32 {
        self.susp_rr_raw()
    }
    
    /// Get raw value of susp_rr
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn susp_rr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..12].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of susp_rr
    #[inline(always)]
    pub fn set_susp_rr(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SuspRear::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    
    /// susp_rl
    ///
    /// RL suspension travel in mm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mm"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn susp_rl(&self) -> f32 {
        self.susp_rl_raw()
    }
    
    /// Get raw value of susp_rl
    ///
    /// - Start bit: 12
    /// - Signal size: 12 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn susp_rl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[12..24].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of susp_rl
    #[inline(always)]
    pub fn set_susp_rl(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SuspRear::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[12..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SuspRear {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for SuspRear {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// RESERVED2
///
/// - Standard ID: 259 (0x103)
/// - Size: 0 bytes
///
/// RESERVER FOR SMU mask - DO NOT USE
#[derive(Clone, Copy)]
pub struct Reserved2 {
    raw: [u8; 0],
}

impl Reserved2 {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x103)});
    
    
    /// Construct new RESERVED2 from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 0] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 0] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for Reserved2 {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 0 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 0];
        raw.copy_from_slice(&payload[..0]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Reserved2 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// SuspFront
///
/// - Standard ID: 260 (0x104)
/// - Size: 3 bytes
/// - Transmitter: ATC
#[derive(Clone, Copy)]
pub struct SuspFront {
    raw: [u8; 3],
}

impl SuspFront {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x104)});
    
    pub const SUSP_FR_MIN: f32 = 0_f32;
    pub const SUSP_FR_MAX: f32 = 0_f32;
    pub const SUSP_FL_MIN: f32 = 0_f32;
    pub const SUSP_FL_MAX: f32 = 0_f32;
    
    /// Construct new SuspFront from values
    pub fn new(susp_fr: f32, susp_fl: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_susp_fr(susp_fr)?;
        res.set_susp_fl(susp_fl)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// susp_fr
    ///
    /// FR suspension travel in mm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mm"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn susp_fr(&self) -> f32 {
        self.susp_fr_raw()
    }
    
    /// Get raw value of susp_fr
    ///
    /// - Start bit: 0
    /// - Signal size: 12 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn susp_fr_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..12].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of susp_fr
    #[inline(always)]
    pub fn set_susp_fr(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SuspFront::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..12].store_le(value);
        Ok(())
    }
    
    /// susp_fl
    ///
    /// FL suspension travel in mm
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mm"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn susp_fl(&self) -> f32 {
        self.susp_fl_raw()
    }
    
    /// Get raw value of susp_fl
    ///
    /// - Start bit: 12
    /// - Signal size: 12 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn susp_fl_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[12..24].load_le::<u16>();
        
        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of susp_fl
    #[inline(always)]
    pub fn set_susp_fl(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: SuspFront::MESSAGE_ID });
        }
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[12..24].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for SuspFront {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for SuspFront {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// TempFrontR
///
/// - Standard ID: 261 (0x105)
/// - Size: 3 bytes
/// - Transmitter: ATC
#[derive(Clone, Copy)]
pub struct TempFrontR {
    raw: [u8; 3],
}

impl TempFrontR {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x105)});
    
    pub const TEMP_MOT_POT_FR_MIN: u16 = 0_u16;
    pub const TEMP_MOT_POT_FR_MAX: u16 = 0_u16;
    pub const TEMP_MOT_PRE_FR_MIN: u16 = 0_u16;
    pub const TEMP_MOT_PRE_FR_MAX: u16 = 0_u16;
    
    /// Construct new TempFrontR from values
    pub fn new(temp_mot_pot_fr: u16, temp_mot_pre_fr: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 3] };
        res.set_temp_mot_pot_fr(temp_mot_pot_fr)?;
        res.set_temp_mot_pre_fr(temp_mot_pre_fr)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 3] {
        &self.raw
    }
    
    /// temp_mot_pot_FR
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn temp_mot_pot_fr(&self) -> u16 {
        self.temp_mot_pot_fr_raw()
    }
    
    /// Get raw value of temp_mot_pot_FR
    ///
    /// - Start bit: 0
    /// - Signal size: 10 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_mot_pot_fr_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..10].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_mot_pot_FR
    #[inline(always)]
    pub fn set_temp_mot_pot_fr(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TempFrontR::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TempFrontR::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..10].store_le(value);
        Ok(())
    }
    
    /// temp_mot_pre_FR
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "C"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn temp_mot_pre_fr(&self) -> u16 {
        self.temp_mot_pre_fr_raw()
    }
    
    /// Get raw value of temp_mot_pre_FR
    ///
    /// - Start bit: 10
    /// - Signal size: 10 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn temp_mot_pre_fr_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[10..20].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of temp_mot_pre_FR
    #[inline(always)]
    pub fn set_temp_mot_pre_fr(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: TempFrontR::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: TempFrontR::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[10..20].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for TempFrontR {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 3 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 3];
        raw.copy_from_slice(&payload[..3]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for TempFrontR {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// PressBrake
///
/// - Standard ID: 264 (0x108)
/// - Size: 2 bytes
/// - Transmitter: SMU
///
/// Hydraulic Brakes Pressures
#[derive(Clone, Copy)]
pub struct PressBrake {
    raw: [u8; 2],
}

impl PressBrake {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x108)});
    
    pub const PRESS_FRONT_MIN: f32 = 0_f32;
    pub const PRESS_FRONT_MAX: f32 = 64_f32;
    pub const PRESS_REAR_MIN: f32 = 0_f32;
    pub const PRESS_REAR_MAX: f32 = 64_f32;
    
    /// Construct new PressBrake from values
    pub fn new(press_front: f32, press_rear: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_press_front(press_front)?;
        res.set_press_rear(press_rear)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// press_front
    ///
    /// - Min: 0
    /// - Max: 64
    /// - Unit: "Bar"
    /// - Receivers: EBS, VCU
    #[inline(always)]
    pub fn press_front(&self) -> f32 {
        self.press_front_raw()
    }
    
    /// Get raw value of press_front
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 0.25
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn press_front_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 0.25_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of press_front
    #[inline(always)]
    pub fn set_press_front(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 64_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: PressBrake::MESSAGE_ID });
        }
        let factor = 0.25_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
    /// press_rear
    ///
    /// - Min: 0
    /// - Max: 64
    /// - Unit: "Bar"
    /// - Receivers: EBS, VCU
    #[inline(always)]
    pub fn press_rear(&self) -> f32 {
        self.press_rear_raw()
    }
    
    /// Get raw value of press_rear
    ///
    /// - Start bit: 8
    /// - Signal size: 8 bits
    /// - Factor: 0.25
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn press_rear_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[8..16].load_le::<u8>();
        
        let factor = 0.25_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of press_rear
    #[inline(always)]
    pub fn set_press_rear(&mut self, value: f32) -> Result<(), CanError> {
        if value < 0_f32 || 64_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: PressBrake::MESSAGE_ID });
        }
        let factor = 0.25_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[8..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PressBrake {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for PressBrake {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// InvVolt
///
/// - Standard ID: 288 (0x120)
/// - Size: 2 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct InvVolt {
    raw: [u8; 2],
}

impl InvVolt {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x120)});
    
    pub const CAR_VOLTAGE_MIN: u16 = 0_u16;
    pub const CAR_VOLTAGE_MAX: u16 = 600_u16;
    
    /// Construct new InvVolt from values
    pub fn new(car_voltage: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 2] };
        res.set_car_voltage(car_voltage)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 2] {
        &self.raw
    }
    
    /// car_voltage
    ///
    /// Voltage seen from car side (inverter) in volts
    ///
    /// - Min: 0
    /// - Max: 600
    /// - Unit: "V"
    /// - Receivers: VCU, SW, BMSHV
    #[inline(always)]
    pub fn car_voltage(&self) -> u16 {
        self.car_voltage_raw()
    }
    
    /// Get raw value of car_voltage
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn car_voltage_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of car_voltage
    #[inline(always)]
    pub fn set_car_voltage(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 600_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: InvVolt::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: InvVolt::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for InvVolt {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 2 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 2];
        raw.copy_from_slice(&payload[..2]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for InvVolt {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Pcu
///
/// - Standard ID: 304 (0x130)
/// - Size: 7 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct Pcu {
    raw: [u8; 7],
}

impl Pcu {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x130)});
    
    pub const MODE_MIN: u8 = 0_u8;
    pub const MODE_MAX: u8 = 2_u8;
    pub const PUMP_SPEED_LEFT_MIN: u8 = 0_u8;
    pub const PUMP_SPEED_LEFT_MAX: u8 = 100_u8;
    pub const PUMP_SPEED_RIGHT_MIN: u8 = 0_u8;
    pub const PUMP_SPEED_RIGHT_MAX: u8 = 100_u8;
    pub const FANRAD_SPEED_LEFT_MIN: u8 = 0_u8;
    pub const FANRAD_SPEED_LEFT_MAX: u8 = 100_u8;
    pub const FANRAD_SPEED_RIGHT_MIN: u8 = 0_u8;
    pub const FANRAD_SPEED_RIGHT_MAX: u8 = 100_u8;
    pub const FANBATT_SPEED_LEFT_MIN: u8 = 0_u8;
    pub const FANBATT_SPEED_LEFT_MAX: u8 = 100_u8;
    pub const FANBATT_SPEED_RIGHT_MIN: u8 = 0_u8;
    pub const FANBATT_SPEED_RIGHT_MAX: u8 = 100_u8;
    
    /// Construct new Pcu from values
    pub fn new(mode: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 7] };
        res.set_mode(mode)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 7] {
        &self.raw
    }
    
    /// Get raw value of mode
    ///
    /// - Start bit: 0
    /// - Signal size: 2 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn mode_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..2].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn mode(&mut self) -> Result<PcuModeIndex, CanError> {
        match self.mode_raw() {
            0 => Ok(PcuModeIndex::M0(PcuModeM0{ raw: self.raw })),
            1 => Ok(PcuModeIndex::M1(PcuModeM1{ raw: self.raw })),
            2 => Ok(PcuModeIndex::M2(PcuModeM2{ raw: self.raw })),
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Pcu::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of mode
    #[inline(always)]
    fn set_mode(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 2_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..2].store_le(value);
        Ok(())
    }
    
    /// Set value of mode
    #[inline(always)]
    pub fn set_m0(&mut self, value: PcuModeM0) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(0)?;
        Ok(())
    }
    
    /// Set value of mode
    #[inline(always)]
    pub fn set_m1(&mut self, value: PcuModeM1) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(1)?;
        Ok(())
    }
    
    /// Set value of mode
    #[inline(always)]
    pub fn set_m2(&mut self, value: PcuModeM2) -> Result<(), CanError> {
        let b0 = BitArray::<_, LocalBits>::new(self.raw);
        let b1 = BitArray::<_, LocalBits>::new(value.raw);
        self.raw = b0.bitor(b1).into_inner();
        self.set_mode(2)?;
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Pcu {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 7 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 7];
        raw.copy_from_slice(&payload[..7]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Pcu {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for multiplexed signal Pcu
pub enum PcuModeIndex {
    M0(PcuModeM0),
    M1(PcuModeM1),
    M2(PcuModeM2),
}

#[derive(Default)]
pub struct PcuModeM0 { raw: [u8; 7] }

impl PcuModeM0 {
pub fn new() -> Self { Self { raw: [0u8; 7] } }
/// pump_enable_left
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn pump_enable_left(&self) -> bool {
    self.pump_enable_left_raw()
}

/// Get raw value of pump_enable_left
///
/// - Start bit: 8
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn pump_enable_left_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[8..9].load_le::<u8>();
    
    signal == 1
}

/// Set value of pump_enable_left
#[inline(always)]
pub fn set_pump_enable_left(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[8..9].store_le(value);
    Ok(())
}

/// pump_speed_left
///
/// - Min: 0
/// - Max: 100
/// - Unit: "%"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn pump_speed_left(&self) -> u8 {
    self.pump_speed_left_raw()
}

/// Get raw value of pump_speed_left
///
/// - Start bit: 9
/// - Signal size: 7 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn pump_speed_left_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[9..16].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of pump_speed_left
#[inline(always)]
pub fn set_pump_speed_left(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 100_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[9..16].store_le(value);
    Ok(())
}

/// pump_enable_right
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn pump_enable_right(&self) -> bool {
    self.pump_enable_right_raw()
}

/// Get raw value of pump_enable_right
///
/// - Start bit: 16
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn pump_enable_right_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[16..17].load_le::<u8>();
    
    signal == 1
}

/// Set value of pump_enable_right
#[inline(always)]
pub fn set_pump_enable_right(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[16..17].store_le(value);
    Ok(())
}

/// pump_speed_right
///
/// - Min: 0
/// - Max: 100
/// - Unit: "%"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn pump_speed_right(&self) -> u8 {
    self.pump_speed_right_raw()
}

/// Get raw value of pump_speed_right
///
/// - Start bit: 17
/// - Signal size: 7 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn pump_speed_right_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[17..24].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of pump_speed_right
#[inline(always)]
pub fn set_pump_speed_right(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 100_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[17..24].store_le(value);
    Ok(())
}

/// fanrad_enable_left
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn fanrad_enable_left(&self) -> bool {
    self.fanrad_enable_left_raw()
}

/// Get raw value of fanrad_enable_left
///
/// - Start bit: 24
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn fanrad_enable_left_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[24..25].load_le::<u8>();
    
    signal == 1
}

/// Set value of fanrad_enable_left
#[inline(always)]
pub fn set_fanrad_enable_left(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[24..25].store_le(value);
    Ok(())
}

/// fanrad_speed_left
///
/// - Min: 0
/// - Max: 100
/// - Unit: "%"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn fanrad_speed_left(&self) -> u8 {
    self.fanrad_speed_left_raw()
}

/// Get raw value of fanrad_speed_left
///
/// - Start bit: 25
/// - Signal size: 7 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn fanrad_speed_left_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[25..32].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of fanrad_speed_left
#[inline(always)]
pub fn set_fanrad_speed_left(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 100_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[25..32].store_le(value);
    Ok(())
}

/// fanrad_enable_right
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn fanrad_enable_right(&self) -> bool {
    self.fanrad_enable_right_raw()
}

/// Get raw value of fanrad_enable_right
///
/// - Start bit: 32
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn fanrad_enable_right_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[32..33].load_le::<u8>();
    
    signal == 1
}

/// Set value of fanrad_enable_right
#[inline(always)]
pub fn set_fanrad_enable_right(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[32..33].store_le(value);
    Ok(())
}

/// fanrad_speed_right
///
/// - Min: 0
/// - Max: 100
/// - Unit: "%"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn fanrad_speed_right(&self) -> u8 {
    self.fanrad_speed_right_raw()
}

/// Get raw value of fanrad_speed_right
///
/// - Start bit: 33
/// - Signal size: 7 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn fanrad_speed_right_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[33..40].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of fanrad_speed_right
#[inline(always)]
pub fn set_fanrad_speed_right(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 100_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[33..40].store_le(value);
    Ok(())
}

/// fanbatt_enable_left
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn fanbatt_enable_left(&self) -> bool {
    self.fanbatt_enable_left_raw()
}

/// Get raw value of fanbatt_enable_left
///
/// - Start bit: 40
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn fanbatt_enable_left_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[40..41].load_le::<u8>();
    
    signal == 1
}

/// Set value of fanbatt_enable_left
#[inline(always)]
pub fn set_fanbatt_enable_left(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[40..41].store_le(value);
    Ok(())
}

/// fanbatt_speed_left
///
/// - Min: 0
/// - Max: 100
/// - Unit: "%"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn fanbatt_speed_left(&self) -> u8 {
    self.fanbatt_speed_left_raw()
}

/// Get raw value of fanbatt_speed_left
///
/// - Start bit: 41
/// - Signal size: 7 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn fanbatt_speed_left_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[41..48].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of fanbatt_speed_left
#[inline(always)]
pub fn set_fanbatt_speed_left(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 100_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[41..48].store_le(value);
    Ok(())
}

/// fanbatt_enable_right
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn fanbatt_enable_right(&self) -> bool {
    self.fanbatt_enable_right_raw()
}

/// Get raw value of fanbatt_enable_right
///
/// - Start bit: 48
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn fanbatt_enable_right_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[48..49].load_le::<u8>();
    
    signal == 1
}

/// Set value of fanbatt_enable_right
#[inline(always)]
pub fn set_fanbatt_enable_right(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[48..49].store_le(value);
    Ok(())
}

/// fanbatt_speed_right
///
/// - Min: 0
/// - Max: 100
/// - Unit: "%"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn fanbatt_speed_right(&self) -> u8 {
    self.fanbatt_speed_right_raw()
}

/// Get raw value of fanbatt_speed_right
///
/// - Start bit: 49
/// - Signal size: 7 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn fanbatt_speed_right_raw(&self) -> u8 {
    let signal = self.raw.view_bits::<Lsb0>()[49..56].load_le::<u8>();
    
    let factor = 1;
    u8::from(signal).saturating_mul(factor).saturating_add(0)
}

/// Set value of fanbatt_speed_right
#[inline(always)]
pub fn set_fanbatt_speed_right(&mut self, value: u8) -> Result<(), CanError> {
    if value < 0_u8 || 100_u8 < value {
        return Err(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID });
    }
    let factor = 1;
    let value = value.checked_sub(0)
        .ok_or(CanError::ParameterOutOfRange { message_id: Pcu::MESSAGE_ID })?;
    let value = (value / factor) as u8;
    
    self.raw.view_bits_mut::<Lsb0>()[49..56].store_le(value);
    Ok(())
}

}

#[derive(Default)]
pub struct PcuModeM1 { raw: [u8; 7] }

impl PcuModeM1 {
pub fn new() -> Self { Self { raw: [0u8; 7] } }
/// rf
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn rf(&self) -> bool {
    self.rf_raw()
}

/// Get raw value of rf
///
/// - Start bit: 2
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn rf_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
    
    signal == 1
}

/// Set value of rf
#[inline(always)]
pub fn set_rf(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
    Ok(())
}

}

#[derive(Default)]
pub struct PcuModeM2 { raw: [u8; 7] }

impl PcuModeM2 {
pub fn new() -> Self { Self { raw: [0u8; 7] } }
/// enable_dv
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn enable_dv(&self) -> bool {
    self.enable_dv_raw()
}

/// Get raw value of enable_dv
///
/// - Start bit: 2
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn enable_dv_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[2..3].load_le::<u8>();
    
    signal == 1
}

/// Set value of enable_dv
#[inline(always)]
pub fn set_enable_dv(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[2..3].store_le(value);
    Ok(())
}

/// enable_embedded
///
/// - Min: 0
/// - Max: 1
/// - Unit: "on"
/// - Receivers: Vector__XXX
#[inline(always)]
pub fn enable_embedded(&self) -> bool {
    self.enable_embedded_raw()
}

/// Get raw value of enable_embedded
///
/// - Start bit: 3
/// - Signal size: 1 bits
/// - Factor: 1
/// - Offset: 0
/// - Byte order: LittleEndian
/// - Value type: Unsigned
#[inline(always)]
pub fn enable_embedded_raw(&self) -> bool {
    let signal = self.raw.view_bits::<Lsb0>()[3..4].load_le::<u8>();
    
    signal == 1
}

/// Set value of enable_embedded
#[inline(always)]
pub fn set_enable_embedded(&mut self, value: bool) -> Result<(), CanError> {
    let value = value as u8;
    self.raw.view_bits_mut::<Lsb0>()[3..4].store_le(value);
    Ok(())
}

}


/// Calib
///
/// - Standard ID: 305 (0x131)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct Calib {
    raw: [u8; 1],
}

impl Calib {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x131)});
    
    pub const POSITION_MIN: u8 = 0_u8;
    pub const POSITION_MAX: u8 = 1_u8;
    
    /// Construct new Calib from values
    pub fn new(position: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_position(position)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Get raw value of position
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn position_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn position(&mut self) -> Result<CalibPositionIndex, CanError> {
        match self.position_raw() {
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: Calib::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of position
    #[inline(always)]
    fn set_position(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Calib::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: Calib::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Calib {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Calib {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for multiplexed signal Calib
pub enum CalibPositionIndex {
}


/// CalibAck
///
/// - Standard ID: 306 (0x132)
/// - Size: 1 bytes
/// - Transmitter: ATC
#[derive(Clone, Copy)]
pub struct CalibAck {
    raw: [u8; 1],
}

impl CalibAck {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x132)});
    
    pub const POSITION_MIN: u8 = 0_u8;
    pub const POSITION_MAX: u8 = 1_u8;
    
    /// Construct new CalibAck from values
    pub fn new(position: u8) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_position(position)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// Get raw value of position
    ///
    /// - Start bit: 0
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn position_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Lsb0>()[0..8].load_le::<u8>();
        
        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    pub fn position(&mut self) -> Result<CalibAckPositionIndex, CanError> {
        match self.position_raw() {
            multiplexor => Err(CanError::InvalidMultiplexor { message_id: CalibAck::MESSAGE_ID, multiplexor: multiplexor.into() }),
        }
    }
    /// Set value of position
    #[inline(always)]
    fn set_position(&mut self, value: u8) -> Result<(), CanError> {
        if value < 0_u8 || 1_u8 < value {
            return Err(CanError::ParameterOutOfRange { message_id: CalibAck::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: CalibAck::MESSAGE_ID })?;
        let value = (value / factor) as u8;
        
        self.raw.view_bits_mut::<Lsb0>()[0..8].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for CalibAck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for CalibAck {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for multiplexed signal CalibAck
pub enum CalibAckPositionIndex {
}


/// PcuSwControl
///
/// - Standard ID: 307 (0x133)
/// - Size: 1 bytes
/// - Transmitter: SW
#[derive(Clone, Copy)]
pub struct PcuSwControl {
    raw: [u8; 1],
}

impl PcuSwControl {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x133)});
    
    
    /// Construct new PcuSwControl from values
    pub fn new(pump: bool, fan: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_pump(pump)?;
        res.set_fan(fan)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// pump
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn pump(&self) -> PcuSwControlPump {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        match signal {
            1 => PcuSwControlPump::On,
            0 => PcuSwControlPump::Off,
            _ => PcuSwControlPump::_Other(self.pump_raw()),
        }
    }
    
    /// Get raw value of pump
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pump_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of pump
    #[inline(always)]
    pub fn set_pump(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// fan
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: ""
    /// - Receivers: VCU
    #[inline(always)]
    pub fn fan(&self) -> PcuSwControlFan {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        match signal {
            1 => PcuSwControlFan::On,
            0 => PcuSwControlFan::Off,
            _ => PcuSwControlFan::_Other(self.fan_raw()),
        }
    }
    
    /// Get raw value of fan
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn fan_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of fan
    #[inline(always)]
    pub fn set_fan(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuSwControl {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for PcuSwControl {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
/// Defined values for pump
#[derive(Clone, Copy, PartialEq)]
pub enum PcuSwControlPump {
    On,
    Off,
    _Other(bool),
}

impl From<PcuSwControlPump> for bool {
    fn from(val: PcuSwControlPump) -> bool {
        match val {
            PcuSwControlPump::On => true,
            PcuSwControlPump::Off => false,
            PcuSwControlPump::_Other(x) => x,
        }
    }
}

/// Defined values for fan
#[derive(Clone, Copy, PartialEq)]
pub enum PcuSwControlFan {
    On,
    Off,
    _Other(bool),
}

impl From<PcuSwControlFan> for bool {
    fn from(val: PcuSwControlFan) -> bool {
        match val {
            PcuSwControlFan::On => true,
            PcuSwControlFan::Off => false,
            PcuSwControlFan::_Other(x) => x,
        }
    }
}


/// PcuRfAck
///
/// - Standard ID: 308 (0x134)
/// - Size: 1 bytes
/// - Transmitter: PCU
#[derive(Clone, Copy)]
pub struct PcuRfAck {
    raw: [u8; 1],
}

impl PcuRfAck {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x134)});
    
    
    /// Construct new PcuRfAck from values
    pub fn new(rf_signal_ack: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_rf_signal_ack(rf_signal_ack)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 1] {
        &self.raw
    }
    
    /// rf_signalAck
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "on"
    /// - Receivers: VCU
    #[inline(always)]
    pub fn rf_signal_ack(&self) -> bool {
        self.rf_signal_ack_raw()
    }
    
    /// Get raw value of rf_signalAck
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn rf_signal_ack_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of rf_signalAck
    #[inline(always)]
    pub fn set_rf_signal_ack(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PcuRfAck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for PcuRfAck {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// EmbeddedAliveCheck
///
/// - Standard ID: 310 (0x136)
/// - Size: 0 bytes
/// - Transmitter: VCU
#[derive(Clone, Copy)]
pub struct EmbeddedAliveCheck {
    raw: [u8; 0],
}

impl EmbeddedAliveCheck {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x136)});
    
    
    /// Construct new EmbeddedAliveCheck from values
    pub fn new() -> Result<Self, CanError> {
        let res = Self { raw: [0u8; 0] };
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 0] {
        &self.raw
    }
    
}

impl core::convert::TryFrom<&[u8]> for EmbeddedAliveCheck {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 0 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 0];
        raw.copy_from_slice(&payload[..0]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for EmbeddedAliveCheck {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}

/// Lem
///
/// - Standard ID: 962 (0x3c2)
/// - Size: 8 bytes
/// - Transmitter: LEM
#[derive(Clone, Copy)]
pub struct Lem {
    raw: [u8; 8],
}

impl Lem {
    pub const MESSAGE_ID: embedded_can::Id = Id::Standard(unsafe { StandardId::new_unchecked(0x3c2)});
    
    pub const CURRENT_MIN: i32 = 0_i32;
    pub const CURRENT_MAX: i32 = 0_i32;
    
    /// Construct new Lem from values
    pub fn new(current: i32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_current(current)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// current
    ///
    /// Current seen from LEM on car side (PDB)
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mA"
    /// - Receivers: VCU, SW, BMSHV
    #[inline(always)]
    pub fn current(&self) -> i32 {
        self.current_raw()
    }
    
    /// Get raw value of current
    ///
    /// - Start bit: 7
    /// - Signal size: 32 bits
    /// - Factor: 1
    /// - Offset: -2147483648
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn current_raw(&self) -> i32 {
        let signal = self.raw.view_bits::<Msb0>()[0..32].load_be::<u32>();
        
        let factor = 1;
        let signal = signal as i32;
        i32::from(signal).saturating_mul(factor).saturating_sub(2147483648)
    }
    
    /// Set value of current
    #[inline(always)]
    pub fn set_current(&mut self, value: i32) -> Result<(), CanError> {
        if value < 0_i32 || 0_i32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: Lem::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_add(2147483648)
            .ok_or(CanError::ParameterOutOfRange { message_id: Lem::MESSAGE_ID })?;
        let value = (value / factor) as u32;
        
        self.raw.view_bits_mut::<Msb0>()[0..32].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Lem {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Lem {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}


/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanError {
    UnknownMessageId(embedded_can::Id),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: embedded_can::Id,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: embedded_can::Id,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

impl core::fmt::Display for CanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

