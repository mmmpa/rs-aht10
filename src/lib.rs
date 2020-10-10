#[repr(u8)]
enum Address {
    Write = 0x38,
    Read = 0x39,
}

const INITIALIZATION: [u8; 3] = [0b1110_0001, 0b0000_1000, 0b0000_0000];
const TRIGGER_MEASUREMENT: [u8; 3] = [0b1010_1100, 0b0011_0011, 0b0000_0000];
const SOFT_RESET: [u8; 1] = [0b1011_1010];

#[derive(Debug, Eq, PartialEq)]
struct Status(u8);

#[derive(Debug, Eq, PartialEq)]
enum BusyIndication {
    Busy,
    MeasurementAndIdle,
}

#[derive(Debug, Eq, PartialEq)]
enum CurrentWorkingMode {
    Nor,
    Cyc,
    Cmd,
}

#[derive(Debug, Eq, PartialEq)]
enum CalibrationEnable {
    Calibrated,
    NotCalibrated,
}

impl Status {
    pub fn busy_indication(&self) -> BusyIndication {
        match self.0 >> 7 {
            1 => BusyIndication::Busy,
            _ => BusyIndication::MeasurementAndIdle,
        }
    }

    pub fn current_working_mode(&self) -> CurrentWorkingMode {
        match (self.0 >> 5) & 0b11 {
            0b00 => CurrentWorkingMode::Nor,
            0b01 => CurrentWorkingMode::Cyc,
            _ => CurrentWorkingMode::Cmd,
        }
    }

    pub fn calibration_enable(&self) -> CalibrationEnable {
        match (self.0 >> 3) & 0b1 {
            1 => CalibrationEnable::Calibrated,
            _ => CalibrationEnable::NotCalibrated,
        }
    }
}

#[repr(C)]
struct Measurement {
    status: Status,
    hum1: u8,
    hum2: u8,
    hum3_temp1: u8,
    temp2: u8,
    temp3: u8,
}

impl Measurement {
    pub fn is_ready(&self) -> bool {
        self.status.calibration_enable() == CalibrationEnable::Calibrated
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn hum(&self) -> f32 {
        match u32::from_be_bytes([0, self.hum1, self.hum2, self.hum3_temp1]) >> 4 {
            0 => 0.0,
            n => n as f32 * 100.0 / 1048576.0,
        }
    }

    pub fn temp(&self) -> f32 {
        match f32::from_be_bytes([0, self.hum3_temp1 & 0x0f, self.temp2, self.temp3]) {
            0.0 => 0.0,
            n => ((200.0 * n) / 1048576.0) - 50.0,
        }
    }
}

const DATA_COLLECTION_DELAY_MS: u8 = 75;
const SOFT_RESET_DELAY_MS: u8 = 20;

type AhtResult<T> = Result<T, ()>;

trait I2cAdp {
    fn read(&mut self, data: &mut [u8]) -> AhtResult<()>;
    fn write(&mut self, data: &[u8]) -> AhtResult<()>;
}

trait Aht {
    type I2c: I2cAdp;

    fn i2c(&mut self) -> &mut Self::I2c;

    fn init(&mut self) -> AhtResult<()>;
    fn measure(&mut self) -> AhtResult<Measurement>;
}
