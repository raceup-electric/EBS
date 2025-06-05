// Generated code!
#![allow(unused_comparisons, unreachable_patterns)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons)]
#![deny(clippy::integer_arithmetic)]

//! Message definitions from file `"test.dbc"`
//!
//! - Version: `Version("")`

use core::ops::BitOr;
use bitvec::prelude::*;
#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};

/// All messages
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Messages {
    /// VALVE_STATUS
    ValveStatus(ValveStatus),
    /// PRESSURE
    Pressure(Pressure),
    /// FORCE
    Force(Force),
    /// VOLTAGE
    Voltage(Voltage),
    /// RAW
    Raw(Raw),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: u32, payload: &[u8]) -> Result<Self, CanError> {
        use core::convert::TryFrom;
        
        let res = match id {
            16 => Messages::ValveStatus(ValveStatus::try_from(payload)?),
            17 => Messages::Pressure(Pressure::try_from(payload)?),
            18 => Messages::Force(Force::try_from(payload)?),
            19 => Messages::Voltage(Voltage::try_from(payload)?),
            20 => Messages::Raw(Raw::try_from(payload)?),
            n => return Err(CanError::UnknownMessageId(n)),
        };
        Ok(res)
    }
}

/// VALVE_STATUS
///
/// - ID: 16 (0x10)
/// - Size: 1 bytes
#[derive(Clone, Copy)]
pub struct ValveStatus {
    raw: [u8; 1],
}

impl ValveStatus {
    pub const MESSAGE_ID: u32 = 16;
    
    
    /// Construct new VALVE_STATUS from values
    pub fn new(tank_one: bool, tank_two: bool) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 1] };
        res.set_tank_one(tank_one)?;
        res.set_tank_two(tank_two)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// tank_one
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "ON/OFF"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_one(&self) -> bool {
        self.tank_one_raw()
    }
    
    /// Get raw value of tank_one
    ///
    /// - Start bit: 0
    /// - Signal size: 1 bits
    /// - Factor: 0
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_one_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[0..1].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of tank_one
    #[inline(always)]
    pub fn set_tank_one(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[0..1].store_le(value);
        Ok(())
    }
    
    /// tank_two
    ///
    /// - Min: 0
    /// - Max: 1
    /// - Unit: "ON/OFF"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_two(&self) -> bool {
        self.tank_two_raw()
    }
    
    /// Get raw value of tank_two
    ///
    /// - Start bit: 1
    /// - Signal size: 1 bits
    /// - Factor: 0
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_two_raw(&self) -> bool {
        let signal = self.raw.view_bits::<Lsb0>()[1..2].load_le::<u8>();
        
        signal == 1
    }
    
    /// Set value of tank_two
    #[inline(always)]
    pub fn set_tank_two(&mut self, value: bool) -> Result<(), CanError> {
        let value = value as u8;
        self.raw.view_bits_mut::<Lsb0>()[1..2].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ValveStatus {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 1 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 1];
        raw.copy_from_slice(&payload[..1]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for ValveStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("ValveStatus")
                .field("tank_one", &self.tank_one())
                .field("tank_two", &self.tank_two())
            .finish()
        } else {
            f.debug_tuple("ValveStatus").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for ValveStatus {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let tank_one = u.int_in_range(0..=1)? == 1;
        let tank_two = u.int_in_range(0..=1)? == 1;
        ValveStatus::new(tank_one,tank_two).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// PRESSURE
///
/// - ID: 17 (0x11)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Pressure {
    raw: [u8; 8],
}

impl Pressure {
    pub const MESSAGE_ID: u32 = 17;
    
    pub const TANK_PRESS_MIN: f32 = 0_f32;
    pub const TANK_PRESS_MAX: f32 = 0_f32;
    pub const BRAKE_PRESS_MIN: f32 = 0_f32;
    pub const BRAKE_PRESS_MAX: f32 = 0_f32;
    
    /// Construct new PRESSURE from values
    pub fn new(tank_press: f32, brake_press: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_tank_press(tank_press)?;
        res.set_brake_press(brake_press)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// tank_press
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_press(&self) -> f32 {
        self.tank_press_raw()
    }
    
    /// Get raw value of tank_press
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of tank_press
    #[inline(always)]
    pub fn set_tank_press(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 17 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// brake_press
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_press(&self) -> f32 {
        self.brake_press_raw()
    }
    
    /// Get raw value of brake_press
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_press
    #[inline(always)]
    pub fn set_brake_press(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 17 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Pressure {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Pressure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Pressure")
                .field("tank_press", &self.tank_press())
                .field("brake_press", &self.brake_press())
            .finish()
        } else {
            f.debug_tuple("Pressure").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Pressure {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let tank_press = u.float_in_range(0_f32..=0_f32)?;
        let brake_press = u.float_in_range(0_f32..=0_f32)?;
        Pressure::new(tank_press,brake_press).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// FORCE
///
/// - ID: 18 (0x12)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Force {
    raw: [u8; 8],
}

impl Force {
    pub const MESSAGE_ID: u32 = 18;
    
    pub const TANK_FORCE_MIN: f32 = 0_f32;
    pub const TANK_FORCE_MAX: f32 = 0_f32;
    pub const BRAKE_FORCE_MIN: f32 = 0_f32;
    pub const BRAKE_FORCE_MAX: f32 = 0_f32;
    
    /// Construct new FORCE from values
    pub fn new(tank_force: f32, brake_force: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_tank_force(tank_force)?;
        res.set_brake_force(brake_force)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// tank_force
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "N"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_force(&self) -> f32 {
        self.tank_force_raw()
    }
    
    /// Get raw value of tank_force
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_force_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of tank_force
    #[inline(always)]
    pub fn set_tank_force(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 18 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// brake_force
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "N"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_force(&self) -> f32 {
        self.brake_force_raw()
    }
    
    /// Get raw value of brake_force
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_force_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_force
    #[inline(always)]
    pub fn set_brake_force(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 18 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Force {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Force {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Force")
                .field("tank_force", &self.tank_force())
                .field("brake_force", &self.brake_force())
            .finish()
        } else {
            f.debug_tuple("Force").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Force {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let tank_force = u.float_in_range(0_f32..=0_f32)?;
        let brake_force = u.float_in_range(0_f32..=0_f32)?;
        Force::new(tank_force,brake_force).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// VOLTAGE
///
/// - ID: 19 (0x13)
/// - Size: 8 bytes
#[derive(Clone, Copy)]
pub struct Voltage {
    raw: [u8; 8],
}

impl Voltage {
    pub const MESSAGE_ID: u32 = 19;
    
    pub const TANK_VOLT_MIN: f32 = 0_f32;
    pub const TANK_VOLT_MAX: f32 = 0_f32;
    pub const BRAKE_VOLT_MIN: f32 = 0_f32;
    pub const BRAKE_VOLT_MAX: f32 = 0_f32;
    
    /// Construct new VOLTAGE from values
    pub fn new(tank_volt: f32, brake_volt: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_tank_volt(tank_volt)?;
        res.set_brake_volt(brake_volt)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// tank_volt
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_volt(&self) -> f32 {
        self.tank_volt_raw()
    }
    
    /// Get raw value of tank_volt
    ///
    /// - Start bit: 0
    /// - Signal size: 32 bits
    /// - Factor: 1.00001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..32].load_le::<u32>();
        
        let factor = 1.00001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of tank_volt
    #[inline(always)]
    pub fn set_tank_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 19 });
        }
        let factor = 1.00001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[0..32].store_le(value);
        Ok(())
    }
    
    /// brake_volt
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "V"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_volt(&self) -> f32 {
        self.brake_volt_raw()
    }
    
    /// Get raw value of brake_volt
    ///
    /// - Start bit: 32
    /// - Signal size: 32 bits
    /// - Factor: 0.00001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_volt_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..64].load_le::<u32>();
        
        let factor = 0.00001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_volt
    #[inline(always)]
    pub fn set_brake_volt(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 19 });
        }
        let factor = 0.00001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u32;
        
        self.raw.view_bits_mut::<Lsb0>()[32..64].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Voltage {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Voltage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Voltage")
                .field("tank_volt", &self.tank_volt())
                .field("brake_volt", &self.brake_volt())
            .finish()
        } else {
            f.debug_tuple("Voltage").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Voltage {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let tank_volt = u.float_in_range(0_f32..=0_f32)?;
        let brake_volt = u.float_in_range(0_f32..=0_f32)?;
        Voltage::new(tank_volt,brake_volt).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// RAW
///
/// - ID: 20 (0x14)
/// - Size: 4 bytes
#[derive(Clone, Copy)]
pub struct Raw {
    raw: [u8; 4],
}

impl Raw {
    pub const MESSAGE_ID: u32 = 20;
    
    pub const TANK_RAW_MIN: u16 = 0_u16;
    pub const TANK_RAW_MAX: u16 = 0_u16;
    pub const BRAKE_RAW_MIN: u16 = 0_u16;
    pub const BRAKE_RAW_MAX: u16 = 0_u16;
    
    /// Construct new RAW from values
    pub fn new(tank_raw: u16, brake_raw: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 4] };
        res.set_tank_raw(tank_raw)?;
        res.set_brake_raw(brake_raw)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// tank_raw
    ///
    /// adc value
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_raw(&self) -> u16 {
        self.tank_raw_raw()
    }
    
    /// Get raw value of tank_raw
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_raw_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        signal
    }
    
    /// Set value of tank_raw
    #[inline(always)]
    pub fn set_tank_raw(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 20 });
        }
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// brake_raw
    ///
    /// adc value
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_raw(&self) -> u16 {
        self.brake_raw_raw()
    }
    
    /// Get raw value of brake_raw
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_raw_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        signal
    }
    
    /// Set value of brake_raw
    #[inline(always)]
    pub fn set_brake_raw(&mut self, value: u16) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_u16 || 0_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 20 });
        }
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for Raw {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 4 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 4];
        raw.copy_from_slice(&payload[..4]);
        Ok(Self { raw })
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Raw {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Raw")
                .field("tank_raw", &self.tank_raw())
                .field("brake_raw", &self.brake_raw())
            .finish()
        } else {
            f.debug_tuple("Raw").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Raw {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let tank_raw = u.int_in_range(0..=0)?;
        let brake_raw = u.int_in_range(0..=0)?;
        Raw::new(tank_raw,brake_raw).map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}


/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(any(feature = "debug", feature = "std"), derive(Debug))]
pub enum CanError {
    UnknownMessageId(u32),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: u32,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: u32,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
impl fmt::Display for CanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl Error for CanError {}
#[cfg(feature = "arb")]
trait UnstructuredFloatExt {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;
}

#[cfg(feature = "arb")]
impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {
        let min = range.start();
        let max = range.end();
        let steps = u32::MAX;
        let factor = (max - min) / (steps as f32);
        let random_int: u32 = self.int_in_range(0..=steps)?;
        let random = min + factor * (random_int as f32);
        Ok(random)
    }
}

