use crate::Status;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Measurement {
    status: Status,
    hum1: u8,
    hum2: u8,
    hum3_temp1: u8,
    temp2: u8,
    temp3: u8,
}

impl Measurement {
    pub fn is_ready(&self) -> bool {
        self.status.is_ready()
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
        match u32::from_be_bytes([0, self.hum3_temp1 & 0x0f, self.temp2, self.temp3]) {
            0 => 0.0,
            n => ((200.0 * n as f32) / 1048576.0) - 50.0,
        }
    }
}

impl From<[u8; 6]> for Measurement {
    fn from(v: [u8; 6]) -> Self {
        unsafe { core::mem::transmute(v) }
    }
}
