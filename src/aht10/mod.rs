mod measurement;
mod result;
mod status;

pub use measurement::*;
pub use result::*;
pub use status::*;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum RegisterAddress {
    Initialization = 0b1110_0001,
    Normalization = 0b1010_1000,
    TriggerMeasurement = 0b1010_1100,
    SoftReset = 0b1011_1010,
    MeasurementResult = 0,
}

const INITIALIZATION_DATA: [u8; 2] = [0b0000_1000, 0b0000_0000];
const NORMALIZATION_DATA: [u8; 2] = [0b0000_0000, 0b0000_0000];
const TRIGGER_MEASUREMENT_DATA: [u8; 2] = [0b0011_0011, 0b0000_0000];

pub const INITIALIZATION_DELAY_MS: u16 = 500;
pub const DATA_COLLECTION_DELAY_MS: u8 = 75;
pub const SOFT_RESET_DELAY_MS: u8 = 20;

pub trait I2c {
    fn read(&mut self, reg: RegisterAddress, data: &mut [u8]) -> Aht10Result<()>;
    fn write(&mut self, reg: RegisterAddress, data: &[u8]) -> Aht10Result<()>;
}

pub trait Aht10 {
    type I2c: I2c;

    fn i2c(&mut self) -> &mut Self::I2c;

    fn initialize(&mut self) -> Aht10Result<()> {
        self.i2c()
            .write(RegisterAddress::Initialization, &INITIALIZATION_DATA)?;

        Ok(())
    }

    fn normalize(&mut self) -> Aht10Result<()> {
        self.i2c()
            .write(RegisterAddress::Normalization, &NORMALIZATION_DATA)?;

        Ok(())
    }

    fn trigger_measure(&mut self) -> Aht10Result<()> {
        self.i2c().write(
            RegisterAddress::TriggerMeasurement,
            &TRIGGER_MEASUREMENT_DATA,
        )?;

        Ok(())
    }

    fn measure(&mut self) -> Aht10Result<Measurement> {
        let mut data = [0; 6];
        self.i2c()
            .read(RegisterAddress::MeasurementResult, &mut data)?;

        Ok(data.into())
    }

    fn reset(&mut self) -> Aht10Result<()> {
        self.i2c().write(RegisterAddress::SoftReset, &[])?;

        Ok(())
    }
}
