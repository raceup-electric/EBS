pub const BUFFER_SIZE: usize = 20;

/// Buffer circolare per filtrare dati adc con Moving Average
pub struct FilterBuffer {
    buffer: [u16; BUFFER_SIZE],
    index: usize,
    sum: u32,
}

impl FilterBuffer {

    pub fn new() -> Self {
        Self {
            buffer: [0; BUFFER_SIZE],
            index: 0,
            sum: 0,
        }
    }
    
    pub fn add(&mut self, value: u16) {
        self.sum -= self.buffer[self.index] as u32;
        self.buffer[self.index] = value;
        self.sum += value as u32;
        self.index = (self.index + 1) % BUFFER_SIZE;
    }

    pub fn avg(&self) -> u16 {
        (self.sum / BUFFER_SIZE as u32) as u16
    }
}
