// Generated code!
#![allow(unused_comparisons, unreachable_patterns, unused_imports)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::useless_conversion, clippy::unnecessary_cast)]
#![allow(clippy::excessive_precision, clippy::manual_range_contains, clippy::absurd_extreme_comparisons, clippy::too_many_arguments)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"example.dbc"`
//!
//! - Version: `Version("")`

use core::ops::BitOr;
use bitvec::prelude::*;
use embedded_can::{Id, StandardId, ExtendedId};

/// All messages
#[derive(Clone)]
pub enum Messages {
    /// PORT_SONAR
    PortSonar(PortSonar),
    /// STARBOARD_SONAR
    StarboardSonar(StarboardSonar),
    /// IMU_ACCELEROMETER
    ImuAccelerometer(ImuAccelerometer),
    /// IMU_GYRO
    ImuGyro(ImuGyro),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        
        let res = match id {
            PortSonar::MESSAGE_ID => Messages::PortSonar(PortSonar::try_from(payload)?),
            StarboardSonar::MESSAGE_ID => Messages::StarboardSonar(StarboardSonar::try_from(payload)?),
            ImuAccelerometer::MESSAGE_ID => Messages::ImuAccelerometer(ImuAccelerometer::try_from(payload)?),
            ImuGyro::MESSAGE_ID => Messages::ImuGyro(ImuGyro::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// PORT_SONAR
///
/// - Extended ID: 2048 (0x800)
/// - Size: 8 bytes
/// - Transmitter: SENSOR_MODULE
#[derive(Clone, Copy)]
pub struct PortSonar {
    raw: [u8; 8],
}

impl PortSonar {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x800)});
    
    pub const PORT_SONAR_HEIGHT_MIN: u16 = 0_u16;
    pub const PORT_SONAR_HEIGHT_MAX: u16 = 6000_u16;
    
    /// Construct new PORT_SONAR from values
    pub fn new(port_sonar_height: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_port_sonar_height(port_sonar_height)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// PORT_SONAR_HEIGHT
    ///
    /// - Min: 0
    /// - Max: 6000
    /// - Unit: "mm"
    /// - Receivers: SENSOR_MODULE
    #[inline(always)]
    pub fn port_sonar_height(&self) -> u16 {
        self.port_sonar_height_raw()
    }
    
    /// Get raw value of PORT_SONAR_HEIGHT
    ///
    /// - Start bit: 1
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn port_sonar_height_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Msb0>()[6..22].load_be::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of PORT_SONAR_HEIGHT
    #[inline(always)]
    pub fn set_port_sonar_height(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 6000_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: PortSonar::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: PortSonar::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[6..22].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for PortSonar {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for PortSonar {
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

/// STARBOARD_SONAR
///
/// - Extended ID: 2049 (0x801)
/// - Size: 8 bytes
/// - Transmitter: SENSOR_MODULE
#[derive(Clone, Copy)]
pub struct StarboardSonar {
    raw: [u8; 8],
}

impl StarboardSonar {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x801)});
    
    pub const STARBOARD_SONAR_HEIGHT_MIN: u16 = 0_u16;
    pub const STARBOARD_SONAR_HEIGHT_MAX: u16 = 6000_u16;
    
    /// Construct new STARBOARD_SONAR from values
    pub fn new(starboard_sonar_height: u16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_starboard_sonar_height(starboard_sonar_height)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// STARBOARD_SONAR_HEIGHT
    ///
    /// - Min: 0
    /// - Max: 6000
    /// - Unit: "mm"
    /// - Receivers: SENSOR_MODULE
    #[inline(always)]
    pub fn starboard_sonar_height(&self) -> u16 {
        self.starboard_sonar_height_raw()
    }
    
    /// Get raw value of STARBOARD_SONAR_HEIGHT
    ///
    /// - Start bit: 1
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn starboard_sonar_height_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Msb0>()[6..22].load_be::<u16>();
        
        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of STARBOARD_SONAR_HEIGHT
    #[inline(always)]
    pub fn set_starboard_sonar_height(&mut self, value: u16) -> Result<(), CanError> {
        if value < 0_u16 || 6000_u16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: StarboardSonar::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: StarboardSonar::MESSAGE_ID })?;
        let value = (value / factor) as u16;
        
        self.raw.view_bits_mut::<Msb0>()[6..22].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for StarboardSonar {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for StarboardSonar {
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

/// IMU_ACCELEROMETER
///
/// - Extended ID: 2050 (0x802)
/// - Size: 8 bytes
/// - Transmitter: SENSOR_MODULE
#[derive(Clone, Copy)]
pub struct ImuAccelerometer {
    raw: [u8; 8],
}

impl ImuAccelerometer {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x802)});
    
    pub const IMU_ACCELERATION_X_MIN: i16 = -40_i16;
    pub const IMU_ACCELERATION_X_MAX: i16 = 40_i16;
    pub const IMU_ACCELERATION_Y_MIN: i16 = -40_i16;
    pub const IMU_ACCELERATION_Y_MAX: i16 = 40_i16;
    pub const IMU_ACCELERATION_Z_MIN: i16 = -40_i16;
    pub const IMU_ACCELERATION_Z_MAX: i16 = 40_i16;
    
    /// Construct new IMU_ACCELEROMETER from values
    pub fn new(imu_acceleration_x: i16, imu_acceleration_y: i16, imu_acceleration_z: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_imu_acceleration_x(imu_acceleration_x)?;
        res.set_imu_acceleration_y(imu_acceleration_y)?;
        res.set_imu_acceleration_z(imu_acceleration_z)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// IMU_ACCELERATION_X
    ///
    /// m/s^2
    ///
    /// - Min: -40
    /// - Max: 40
    /// - Unit: ""
    /// - Receivers: SENSOR_MODULE
    #[inline(always)]
    pub fn imu_acceleration_x(&self) -> i16 {
        self.imu_acceleration_x_raw()
    }
    
    /// Get raw value of IMU_ACCELERATION_X
    ///
    /// - Start bit: 1
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn imu_acceleration_x_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Msb0>()[6..22].load_be::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IMU_ACCELERATION_X
    #[inline(always)]
    pub fn set_imu_acceleration_x(&mut self, value: i16) -> Result<(), CanError> {
        if value < -40_i16 || 40_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ImuAccelerometer::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ImuAccelerometer::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[6..22].store_be(value);
        Ok(())
    }
    
    /// IMU_ACCELERATION_Y
    ///
    /// m/s^2
    ///
    /// - Min: -40
    /// - Max: 40
    /// - Unit: ""
    /// - Receivers: SENSOR_MODULE
    #[inline(always)]
    pub fn imu_acceleration_y(&self) -> i16 {
        self.imu_acceleration_y_raw()
    }
    
    /// Get raw value of IMU_ACCELERATION_Y
    ///
    /// - Start bit: 17
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn imu_acceleration_y_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Msb0>()[22..38].load_be::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IMU_ACCELERATION_Y
    #[inline(always)]
    pub fn set_imu_acceleration_y(&mut self, value: i16) -> Result<(), CanError> {
        if value < -40_i16 || 40_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ImuAccelerometer::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ImuAccelerometer::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[22..38].store_be(value);
        Ok(())
    }
    
    /// IMU_ACCELERATION_Z
    ///
    /// m/s^2
    ///
    /// - Min: -40
    /// - Max: 40
    /// - Unit: ""
    /// - Receivers: SENSOR_MODULE
    #[inline(always)]
    pub fn imu_acceleration_z(&self) -> i16 {
        self.imu_acceleration_z_raw()
    }
    
    /// Get raw value of IMU_ACCELERATION_Z
    ///
    /// - Start bit: 33
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn imu_acceleration_z_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Msb0>()[38..54].load_be::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IMU_ACCELERATION_Z
    #[inline(always)]
    pub fn set_imu_acceleration_z(&mut self, value: i16) -> Result<(), CanError> {
        if value < -40_i16 || 40_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ImuAccelerometer::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ImuAccelerometer::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[38..54].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ImuAccelerometer {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ImuAccelerometer {
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

/// IMU_GYRO
///
/// - Extended ID: 2051 (0x803)
/// - Size: 8 bytes
/// - Transmitter: SENSOR_MODULE
#[derive(Clone, Copy)]
pub struct ImuGyro {
    raw: [u8; 8],
}

impl ImuGyro {
    pub const MESSAGE_ID: embedded_can::Id = Id::Extended(unsafe { ExtendedId::new_unchecked(0x803)});
    
    pub const IMU_GYRO_ROLL_MIN: i16 = -250_i16;
    pub const IMU_GYRO_ROLL_MAX: i16 = 250_i16;
    pub const IMU_GYRO_PITCH_MIN: i16 = -250_i16;
    pub const IMU_GYRO_PITCH_MAX: i16 = 250_i16;
    pub const IMU_GYRO_YAW_MIN: i16 = -250_i16;
    pub const IMU_GYRO_YAW_MAX: i16 = 250_i16;
    
    /// Construct new IMU_GYRO from values
    pub fn new(imu_gyro_roll: i16, imu_gyro_pitch: i16, imu_gyro_yaw: i16) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_imu_gyro_roll(imu_gyro_roll)?;
        res.set_imu_gyro_pitch(imu_gyro_pitch)?;
        res.set_imu_gyro_yaw(imu_gyro_yaw)?;
        Ok(res)
    }
    
    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }
    
    /// IMU_GYRO_ROLL
    ///
    /// - Min: -250
    /// - Max: 250
    /// - Unit: ""
    /// - Receivers: SENSOR_MODULE
    #[inline(always)]
    pub fn imu_gyro_roll(&self) -> i16 {
        self.imu_gyro_roll_raw()
    }
    
    /// Get raw value of IMU_GYRO_ROLL
    ///
    /// - Start bit: 1
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn imu_gyro_roll_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Msb0>()[6..22].load_be::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IMU_GYRO_ROLL
    #[inline(always)]
    pub fn set_imu_gyro_roll(&mut self, value: i16) -> Result<(), CanError> {
        if value < -250_i16 || 250_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ImuGyro::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ImuGyro::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[6..22].store_be(value);
        Ok(())
    }
    
    /// IMU_GYRO_PITCH
    ///
    /// - Min: -250
    /// - Max: 250
    /// - Unit: ""
    /// - Receivers: SENSOR_MODULE
    #[inline(always)]
    pub fn imu_gyro_pitch(&self) -> i16 {
        self.imu_gyro_pitch_raw()
    }
    
    /// Get raw value of IMU_GYRO_PITCH
    ///
    /// - Start bit: 17
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn imu_gyro_pitch_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Msb0>()[22..38].load_be::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IMU_GYRO_PITCH
    #[inline(always)]
    pub fn set_imu_gyro_pitch(&mut self, value: i16) -> Result<(), CanError> {
        if value < -250_i16 || 250_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ImuGyro::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ImuGyro::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[22..38].store_be(value);
        Ok(())
    }
    
    /// IMU_GYRO_YAW
    ///
    /// - Min: -250
    /// - Max: 250
    /// - Unit: ""
    /// - Receivers: SENSOR_MODULE
    #[inline(always)]
    pub fn imu_gyro_yaw(&self) -> i16 {
        self.imu_gyro_yaw_raw()
    }
    
    /// Get raw value of IMU_GYRO_YAW
    ///
    /// - Start bit: 33
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Signed
    #[inline(always)]
    pub fn imu_gyro_yaw_raw(&self) -> i16 {
        let signal = self.raw.view_bits::<Msb0>()[38..54].load_be::<i16>();
        
        let factor = 1;
        let signal = signal as i16;
        i16::from(signal).saturating_mul(factor).saturating_add(0)
    }
    
    /// Set value of IMU_GYRO_YAW
    #[inline(always)]
    pub fn set_imu_gyro_yaw(&mut self, value: i16) -> Result<(), CanError> {
        if value < -250_i16 || 250_i16 < value {
            return Err(CanError::ParameterOutOfRange { message_id: ImuGyro::MESSAGE_ID });
        }
        let factor = 1;
        let value = value.checked_sub(0)
            .ok_or(CanError::ParameterOutOfRange { message_id: ImuGyro::MESSAGE_ID })?;
        let value = (value / factor) as i16;
        
        let value = u16::from_ne_bytes(value.to_ne_bytes());
        self.raw.view_bits_mut::<Msb0>()[38..54].store_be(value);
        Ok(())
    }
    
}

impl core::convert::TryFrom<&[u8]> for ImuGyro {
    type Error = CanError;
    
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 { return Err(CanError::InvalidPayloadSize); }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for ImuGyro {
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

