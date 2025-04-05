use embassy_stm32::can::{frame::Envelope, Frame, Id, StandardId};
use defmt::unwrap;

#[derive(Clone)]
pub struct CanFrame {
    id: u16,
    data: [u8; 8],
    len: usize,
    frame: Frame
}

impl CanFrame {
    pub fn new(id: u16, data: &[u8]) -> Self {
        let mut frame_data = [0u8; 8]; 
        let len = data.len().min(8);

        frame_data[..len].copy_from_slice(&data[..len]);

        let tx_frame = Frame::new_data(unwrap!(StandardId::new(id as _)), data).unwrap();

        CanFrame {
            id,
            data: frame_data,
            len,
            frame: tx_frame
        }
    }

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
            len: rx_frame.header().len() as usize,
            frame: rx_frame
        }
    }

    pub fn frame(&self) -> Frame{
        self.frame
    }

    pub fn _bytes(&self) -> [u8; 8] {
        self.data
    }

    pub fn byte(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

pub fn canframe_to_message(frame: &CanFrame) -> Result<Messages, CanError> {
    Messages::from_can_message(frame.id() as u32, &frame._bytes())
}
pub fn message_to_canframe(message: &Messages) -> CanFrame {
    let id = match message {
        Messages::TanksEbs(_) => TanksEbs::MESSAGE_ID,
        Messages::PcuFault(_) => PcuFault::MESSAGE_ID,
        Messages::Paddle(_) => Paddle::MESSAGE_ID,
        // Aggiungi altri messaggi qui...
        _ => 0, // Valore placeholder, se serve gestire errori metti un `panic!`
    };

    CanFrame::new(id as u16, message.raw())
}
