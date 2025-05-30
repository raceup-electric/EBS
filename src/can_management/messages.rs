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
    
    pub const TANK_ONE_PRESS_MIN: f32 = 0_f32;
    pub const TANK_ONE_PRESS_MAX: f32 = 0_f32;
    pub const TANK_TWO_PRESS_MIN: f32 = 0_f32;
    pub const TANK_TWO_PRESS_MAX: f32 = 0_f32;
    pub const BRAKE_FRONT_PRESS_MIN: f32 = 0_f32;
    pub const BRAKE_FRONT_PRESS_MAX: f32 = 0_f32;
    pub const BRAKE_REAR_PRESS_MIN: f32 = 0_f32;
    pub const BRAKE_REAR_PRESS_MAX: f32 = 0_f32;
    
    /// Construct new PRESSURE from values
    pub fn new(tank_one_press: f32, tank_two_press: f32, brake_front_press: f32, brake_rear_press: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_tank_one_press(tank_one_press)?;
        res.set_tank_two_press(tank_two_press)?;
        res.set_brake_front_press(brake_front_press)?;
        res.set_brake_rear_press(brake_rear_press)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// tank_one_press
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_one_press(&self) -> f32 {
        self.tank_one_press_raw()
    }
    
    /// Get raw value of tank_one_press
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_one_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of tank_one_press
    #[inline(always)]
    pub fn set_tank_one_press(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 17 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// tank_two_press
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_two_press(&self) -> f32 {
        self.tank_two_press_raw()
    }
    
    /// Get raw value of tank_two_press
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_two_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of tank_two_press
    #[inline(always)]
    pub fn set_tank_two_press(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 17 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// brake_front_press
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_front_press(&self) -> f32 {
        self.brake_front_press_raw()
    }
    
    /// Get raw value of brake_front_press
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_front_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_front_press
    #[inline(always)]
    pub fn set_brake_front_press(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 17 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// brake_rear_press
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Bar"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_rear_press(&self) -> f32 {
        self.brake_rear_press_raw()
    }
    
    /// Get raw value of brake_rear_press
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.001
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_rear_press_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_rear_press
    #[inline(always)]
    pub fn set_brake_rear_press(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 17 });
        }
        let factor = 0.001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
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
                .field("tank_one_press", &self.tank_one_press())
                .field("tank_two_press", &self.tank_two_press())
                .field("brake_front_press", &self.brake_front_press())
                .field("brake_rear_press", &self.brake_rear_press())
            .finish()
        } else {
            f.debug_tuple("Pressure").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Pressure {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let tank_one_press = u.float_in_range(0_f32..=0_f32)?;
        let tank_two_press = u.float_in_range(0_f32..=0_f32)?;
        let brake_front_press = u.float_in_range(0_f32..=0_f32)?;
        let brake_rear_press = u.float_in_range(0_f32..=0_f32)?;
        Pressure::new(tank_one_press,tank_two_press,brake_front_press,brake_rear_press).map_err(|_| arbitrary::Error::IncorrectFormat)
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
    
    pub const TANK_ONE_FORCE_MIN: f32 = 0_f32;
    pub const TANK_ONE_FORCE_MAX: f32 = 0_f32;
    pub const TANK_TWO_FORCE_MIN: f32 = 0_f32;
    pub const TANK_TWO_FORCE_MAX: f32 = 0_f32;
    pub const BRAKE_REAR_FORCE_MIN: f32 = 0_f32;
    pub const BRAKE_REAR_FORCE_MAX: f32 = 0_f32;
    pub const BRAKE_FRONT_FORCE_MIN: f32 = 0_f32;
    pub const BRAKE_FRONT_FORCE_MAX: f32 = 0_f32;
    
    /// Construct new FORCE from values
    pub fn new(tank_one_force: f32, tank_two_force: f32, brake_rear_force: f32, brake_front_force: f32) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_tank_one_force(tank_one_force)?;
        res.set_tank_two_force(tank_two_force)?;
        res.set_brake_rear_force(brake_rear_force)?;
        res.set_brake_front_force(brake_front_force)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
    
    /// tank_one_force
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "N"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_one_force(&self) -> f32 {
        self.tank_one_force_raw()
    }
    
    /// Get raw value of tank_one_force
    ///
    /// - Start bit: 0
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_one_force_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[0..16].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of tank_one_force
    #[inline(always)]
    pub fn set_tank_one_force(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 18 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[0..16].store_le(value);
        Ok(())
    }
    
    /// tank_two_force
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "N"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn tank_two_force(&self) -> f32 {
        self.tank_two_force_raw()
    }
    
    /// Get raw value of tank_two_force
    ///
    /// - Start bit: 16
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn tank_two_force_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[16..32].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of tank_two_force
    #[inline(always)]
    pub fn set_tank_two_force(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 18 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[16..32].store_le(value);
        Ok(())
    }
    
    /// brake_rear_force
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "N"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_rear_force(&self) -> f32 {
        self.brake_rear_force_raw()
    }
    
    /// Get raw value of brake_rear_force
    ///
    /// - Start bit: 32
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_rear_force_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[32..48].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_rear_force
    #[inline(always)]
    pub fn set_brake_rear_force(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 18 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[32..48].store_le(value);
        Ok(())
    }
    
    /// brake_front_force
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "N"
    /// - Receivers: Vector__XXX
    #[inline(always)]
    pub fn brake_front_force(&self) -> f32 {
        self.brake_front_force_raw()
    }
    
    /// Get raw value of brake_front_force
    ///
    /// - Start bit: 48
    /// - Signal size: 16 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: LittleEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn brake_front_force_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Lsb0>()[48..64].load_le::<u16>();
        
        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }
    
    /// Set value of brake_front_force
    #[inline(always)]
    pub fn set_brake_front_force(&mut self, value: f32) -> Result<(), CanError> {
        #[cfg(feature = "range_checked")]
        if value < 0_f32 || 0_f32 < value {
            return Err(CanError::ParameterOutOfRange { message_id: 18 });
        }
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;
        
        self.raw.view_bits_mut::<Lsb0>()[48..64].store_le(value);
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
                .field("tank_one_force", &self.tank_one_force())
                .field("tank_two_force", &self.tank_two_force())
                .field("brake_rear_force", &self.brake_rear_force())
                .field("brake_front_force", &self.brake_front_force())
            .finish()
        } else {
            f.debug_tuple("Force").field(&self.raw).finish()
        }
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Force {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let tank_one_force = u.float_in_range(0_f32..=0_f32)?;
        let tank_two_force = u.float_in_range(0_f32..=0_f32)?;
        let brake_rear_force = u.float_in_range(0_f32..=0_f32)?;
        let brake_front_force = u.float_in_range(0_f32..=0_f32)?;
        Force::new(tank_one_force,tank_two_force,brake_rear_force,brake_front_force).map_err(|_| arbitrary::Error::IncorrectFormat)
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

