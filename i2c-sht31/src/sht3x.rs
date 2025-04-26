use bitflags::bitflags;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

#[derive(Debug, Clone)]
pub struct Sht3x<D> {
    address: Address,
    delay: D,
}

// 2.2 Timing Specification for the Sensor System
// Table 4
// TODO: Support longer times needed with lower voltage (Table 5).
const SOFT_RESET_TIME_MS: u8 = 1;

// 4: Operation and Communication
const COMMAND_WAIT_TIME_MS: u8 = 1;

impl<D: DelayNs> Sht3x<D> {
    /// Creates a new driver.
    pub const fn new(address: Address, delay: D) -> Self {
        Self { address, delay }
    }

    /// Send an I2C command.
    fn command<I2C: I2c>(
        &mut self,
        i2c: &mut I2C,
        command: Command,
        wait_time: Option<u8>,
    ) -> Result<(), Error<I2C::Error>> {
        let cmd_bytes = command.value().to_be_bytes();
        i2c.write(self.address as u8, &cmd_bytes)
            .map_err(Error::I2c)?;

        self.delay
            .delay_ms(wait_time.unwrap_or(0).max(COMMAND_WAIT_TIME_MS).into());

        Ok(())
    }

    /// Take a temperature and humidity measurement.
    pub fn measure<I2C: I2c>(
        &mut self,
        i2c: &mut I2C,
        cs: ClockStretch,
        rpt: Repeatability,
    ) -> Result<Measurement, Error<I2C::Error>> {
        self.command(i2c, Command::SingleShot(cs, rpt), Some(rpt.max_duration()))?;
        let mut buf = [0; 6];
        i2c.read(self.address as u8, &mut buf).map_err(Error::I2c)?;

        let temperature = check_crc::<I2C>([buf[0], buf[1]], buf[2]).map(convert_temperature)?;
        let humidity = check_crc::<I2C>([buf[3], buf[4]], buf[5]).map(convert_humidity)?;

        Ok(Measurement {
            temperature,
            humidity,
        })
    }

    /// Soft reset the sensor.
    pub fn reset<I2C: I2c>(&mut self, i2c: &mut I2C) -> Result<(), Error<I2C::Error>> {
        self.command(i2c, Command::SoftReset, Some(SOFT_RESET_TIME_MS))
    }

    /// Read the status register.
    pub fn status<I2C: I2c>(&mut self, i2c: &mut I2C) -> Result<Status, Error<I2C::Error>> {
        self.command(i2c, Command::Status, None)?;
        let mut buf = [0; 3];
        i2c.read(self.address as u8, &mut buf).map_err(Error::I2c)?;

        let status = check_crc::<I2C>([buf[0], buf[1]], buf[2])?;
        Ok(Status::from_bits_truncate(status))
    }

    /// Clear the status register.
    pub fn clear_status<I2C: I2c>(&mut self, i2c: &mut I2C) -> Result<(), Error<I2C::Error>> {
        self.command(i2c, Command::ClearStatus, None)
    }
}

const fn convert_temperature(raw: u16) -> i32 {
    -4500 + (17500 * raw as i32) / 65535
}

const fn convert_humidity(raw: u16) -> u16 {
    ((10000 * raw as u32) / 65535) as u16
}

/// Compare the CRC of the input array to the given CRC checksum.
fn check_crc<I2C: I2c>(data: [u8; 2], crc: u8) -> Result<u16, Error<I2C::Error>> {
    let calculated_crc = crc8(data);

    if calculated_crc == crc {
        Ok(u16::from_be_bytes(data))
    } else {
        Err(Error::Crc)
    }
}

/// Calculate the CRC8 checksum for the given input array.
fn crc8(data: [u8; 2]) -> u8 {
    let mut crc: u8 = 0xff;

    for byte in data {
        crc ^= byte;

        for _ in 0..8 {
            if crc & 0x80 > 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        }
    }

    crc
}

/// Errors
#[derive(Debug)]
pub enum Error<E> {
    /// Wrong CRC
    Crc,
    /// I2C bus error
    I2c(E),
}

/// I2C address
#[derive(Debug, Copy, Clone)]
pub enum Address {
    /// Address pin held high
    High = 0x45,
    /// Address pin held low
    Low = 0x44,
}

/// Clock stretching
#[derive(Debug)]
pub enum ClockStretch {
    Enabled,
    Disabled,
}

/// Periodic data acquisition rate
#[allow(non_camel_case_types, unused)]
enum Rate {
    /// 0.5 measurements per second
    R0_5,
    /// 1 measurement per second
    R1,
    /// 2 measurements per second
    R2,
    /// 4 measurements per second
    R4,
    /// 10 measurements per second
    R10,
}

#[derive(Copy, Clone)]
pub enum Repeatability {
    High,
    Medium,
    Low,
}

impl Repeatability {
    /// Maximum measurement duration in milliseconds
    const fn max_duration(&self) -> u8 {
        match *self {
            Repeatability::Low => 4,
            Repeatability::Medium => 6,
            Repeatability::High => 15,
        }
    }
}

#[allow(unused)]
enum Command {
    SingleShot(ClockStretch, Repeatability),
    Periodic(Rate, Repeatability),
    FetchData,
    PeriodicWithART,
    Break,
    SoftReset,
    HeaterEnable,
    HeaterDisable,
    Status,
    ClearStatus,
}

impl Command {
    const fn value(&self) -> u16 {
        use ClockStretch::Disabled as CSDisabled;
        use ClockStretch::Enabled as CSEnabled;
        use Rate::*;
        use Repeatability::*;
        match *self {
            // 4.3 Measurement Commands for Single Shot Data Acquisition Mode
            // Table 8
            Command::SingleShot(CSEnabled, High) => 0x2C06,
            Command::SingleShot(CSEnabled, Medium) => 0x2C0D,
            Command::SingleShot(CSEnabled, Low) => 0x2C10,
            Command::SingleShot(CSDisabled, High) => 0x2400,
            Command::SingleShot(CSDisabled, Medium) => 0x240B,
            Command::SingleShot(CSDisabled, Low) => 0x2416,

            // 4.5 Measurement Commands for Periodic Data Acquisition Mode
            // Table 9
            Command::Periodic(R0_5, High) => 0x2032,
            Command::Periodic(R0_5, Medium) => 0x2024,
            Command::Periodic(R0_5, Low) => 0x202F,
            Command::Periodic(R1, High) => 0x2130,
            Command::Periodic(R1, Medium) => 0x2126,
            Command::Periodic(R1, Low) => 0x212D,
            Command::Periodic(R2, High) => 0x2236,
            Command::Periodic(R2, Medium) => 0x2220,
            Command::Periodic(R2, Low) => 0x222B,
            Command::Periodic(R4, High) => 0x2334,
            Command::Periodic(R4, Medium) => 0x2322,
            Command::Periodic(R4, Low) => 0x2329,
            Command::Periodic(R10, High) => 0x2737,
            Command::Periodic(R10, Medium) => 0x2721,
            Command::Periodic(R10, Low) => 0x272A,

            // 4.6 Readout of Measurement Results for Periodic Mode
            // Table 10
            Command::FetchData => 0xE000,

            // 4.7 ART command
            // Table 11
            Command::PeriodicWithART => 0x2B32,

            // 4.8 Break command
            // Table 12
            Command::Break => 0x3093,

            // 4.9 Reset
            // Table 13
            Command::SoftReset => 0x30A2,

            // 4.10 Heater
            // Table 15
            Command::HeaterEnable => 0x306D,
            Command::HeaterDisable => 0x3066,

            // 4.11 Status register
            // Table 16
            Command::Status => 0xF32D,
            // Table 18
            Command::ClearStatus => 0x3041,
        }
    }
}

#[derive(Debug)]
pub struct Measurement {
    pub temperature: i32,
    pub humidity: u16,
}

bitflags! {
    /// Status register
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Status: u16 {
        /// Alert pending status
        const ALERT_PENDING         = 1 << 15;
        /// Heater status
        const HEATER                = 1 << 13;
        /// RH tracking alert
        const RH_TRACKING_ALERT     = 1 << 11;
        /// T tracking alert
        const T_TRACKING_ALERT      = 1 << 10;
        /// System reset detected
        const SYSTEM_RESET_DETECTED = 1 <<  4;
        /// Command status
        const COMMAND               = 1 <<  1;
        /// Write data checksum status
        const WRITE_DATA_CHECKSUM   = 1 <<  0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc() {
        assert_eq!(crc8([0xBE, 0xEF]), 0x92);
    }
}
