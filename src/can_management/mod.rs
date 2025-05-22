pub mod frame;
pub mod can_controller;
pub mod messages;
pub mod can;
pub mod can_service;

pub use can_controller::CanError;
use defmt::info;
pub use frame::CanFrame;
pub use can_controller::CanController;


#[macro_export]
macro_rules! get_byte {
    ($value:expr, $byte_num:expr) => {
        (($value >> ($byte_num * 8)) & 0xFF) as u8
    };
    
    ($array:expr, $byte_num:expr, slice) => {
        $array.get($byte_num).copied().unwrap_or(0)
    };
}


pub async fn can_operation(can: &mut CanController<'_>, sens1: u16, sens2: u16) {
    let can_first: [u8; 8] = [
        get_byte!(sens1, 0),
        get_byte!(sens1, 1),
        get_byte!(sens2, 0),
        get_byte!(sens2, 1),
        0, 0, 0, 0
    ];

    let frame_send = CanFrame::new(32,&can_first);
    match can.write(&frame_send).await {
        Ok(_) => {
            info!("Message sent! {}", &frame_send.id());
            for i in 0..frame_send.len() {
                info!("Byte: {}: {}", i, &frame_send.byte(i));
            }
        }

        Err(CanError::Timeout) => {
            info!("Timeout Can connection");
        }

        Err(_) => {
            info!("Can write error");
        }
    }

    let can_second = [
        0, 0
    ];

    let frame_send = CanFrame::new(32, &can_second);
    match can.write(&frame_send).await {
        Ok(_) => {
            info!("Message sent! {}", &frame_send.id());
            for i in 0..frame_send.len() {
                info!("Byte: {}: {}", i, &frame_send.byte(i));
            }
        }

        Err(CanError::Timeout) => {
            info!("Timeout Can connection");
        }

        Err(_) => {
            info!("Can write error");
        }
    }
}