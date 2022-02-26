use crate::*;
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use std::path::Path;

pub struct Aht10Client {
    core: Aht10CoreClient,
}

impl Aht10Client {
    pub fn new(i2c_cli: LinuxI2CDevice) -> Self {
        let core = Aht10CoreClient { i2c_cli };
        Self { core }
    }

    pub fn new_with_path_and_address_hex<P: AsRef<Path>>(
        path: P,
        address_hex: &str,
    ) -> Aht10Result<Self> {
        let address = u16::from_str_radix(&address_hex[2..], 16).unwrap();
        debug!("address: {}", address);

        let i2c_cli = LinuxI2CDevice::new(path, address)?;
        let core = Aht10CoreClient { i2c_cli };

        Ok(Self { core })
    }
}

pub struct Aht10CoreClient {
    i2c_cli: LinuxI2CDevice,
}

impl I2c for Aht10CoreClient {
    fn read(&mut self, reg: RegisterAddress, data: &mut [u8]) -> Aht10Result<()> {
        let re = self
            .i2c_cli
            .smbus_read_i2c_block_data(reg as u8, data.len() as u8)?;
        for i in 0..data.len() {
            data[i] = re[i];
        }
        Ok(())
    }

    fn write(&mut self, reg: RegisterAddress, data: &[u8]) -> Aht10Result<()> {
        self.i2c_cli.smbus_write_i2c_block_data(reg as u8, data)?;
        Ok(())
    }
}

impl Aht10 for Aht10Client {
    type I2c = Aht10CoreClient;

    fn i2c(&mut self) -> &mut Self::I2c {
        &mut self.core
    }
}

impl From<LinuxI2CError> for Aht10Error {
    fn from(e: LinuxI2CError) -> Self {
        Self::I2cError(e.to_string())
    }
}
