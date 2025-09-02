use embassy_stm32::bind_interrupts;
use embassy_stm32::can::filter::Mask32;
use embassy_stm32::can::{
    Can, Fifo, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler
};

use embassy_stm32::{
    peripherals::{ CAN1, CAN2, PA11, PA12, PB12, PB13}
};

bind_interrupts!(struct Irqs1 {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

bind_interrupts!(struct Irqs2 {
    CAN2_RX0 => Rx0InterruptHandler<CAN2>;
    CAN2_RX1 => Rx1InterruptHandler<CAN2>;
    CAN2_SCE => SceInterruptHandler<CAN2>;
    CAN2_TX => TxInterruptHandler<CAN2>;
});

pub struct CanController<'a> {
    pub can: Can<'a>,
    pub is_can2: bool
}


impl<'a> CanController<'a>{
    async fn new(mut controller: CanController<'a>, baudrate: u32) -> Self {
        controller.can.modify_config()
            .set_loopback(false) // Receive own frames
            .set_bitrate(baudrate);
        
        if !controller.is_can2 {controller.can.modify_filters().enable_bank(0, Fifo::Fifo0, Mask32::accept_all());}
        controller.can.enable().await;
        controller
    }

    pub async fn _new_can1(peri: CAN1, rx: PA11, tx: PA12, baudrate: u32) -> Self {
        let controller = CanController {
            can: Can::new(
                peri,
                rx,
                tx,
                Irqs1
            ),
            is_can2: false
        };
        Self::new(controller, baudrate).await
    }

    pub async fn new_can2(peri: CAN2, rx: PB12, tx: PB13, baudrate: u32, peri1: CAN1, mut rx1: PA11, mut tx1: PA12) -> (Self, PA11, PA12) {
        let mut can1 = Can::new(peri1, &mut rx1, &mut tx1, Irqs1);
 
        let controller = CanController {
            can: Can::new(peri, rx, tx, Irqs2),
            is_can2: true
        };

        can1.modify_filters().set_split(0).num_banks();
        can1.modify_filters().slave_filters().enable_bank(0, Fifo::Fifo1, Mask32::accept_all());
        can1.sleep().await;
        drop(can1);

        (Self::new(controller, baudrate).await, rx1, tx1)       
    }

}
