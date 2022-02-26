#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Status(u8);

#[derive(Debug, Eq, PartialEq)]
pub enum BusyIndication {
    Busy,
    MeasurementAndIdle,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CurrentWorkingMode {
    Nor,
    Cyc,
    Cmd,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CalibrationEnable {
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

    pub fn is_ready(&self) -> bool {
        self.calibration_enable() == CalibrationEnable::Calibrated
    }

    pub fn raw(&self) -> u8 {
        self.0
    }
}
