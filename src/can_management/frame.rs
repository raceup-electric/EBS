use embassy_stm32::can::{frame::Envelope, Id};

#[derive(Clone)]
pub struct CanFrame {
    id: u16,
    data: [u8; 8],
}

impl CanFrame {
    pub fn from_envelope(envelope: Envelope) -> Self {
        let rx_frame = envelope.frame;
        let mut frame_data = [0u8; 8]; 
        let len: usize = rx_frame.header().len().min(8) as usize;

        frame_data[..len].copy_from_slice(&rx_frame.data()[..len]);

        let id = match rx_frame.id() {
            Id::Standard(id) => id.as_raw(), 
            Id::Extended(id) => id.standard_id().as_raw(), 
        };

        CanFrame {
            id,
            data: frame_data,
        }
    }

    pub fn _bytes(&self) -> [u8; 8] {
        self.data
    }

    #[allow(dead_code)]
    pub fn byte(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn id(&self) -> u16 {
        self.id
    }
}
