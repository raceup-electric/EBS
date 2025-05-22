use embassy_executor::task;
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use static_cell::StaticCell;
pub struct CanPeripherals {
    pub can2: CAN2,
    pub pb12: PB12,
    pub pb13: PB13,
    pub can1: CAN1,
    pub pa11: PA11,
    pub pa12: PA12,
}


pub fn init_can_peripherals(p: &mut Peripherals) -> CanPeripherals {
    CanPeripherals {
        can2: p.CAN2.take().unwrap(),
        pb12: p.PB12.take().unwrap(),
        pb13: p.PB13.take().unwrap(),
        can1: p.CAN1.take().unwrap(),
        pa11: p.PA11.take().unwrap(),
        pa12: p.PA12.take().unwrap(),
    }
}

pub async fn init_can_service(spawner: &Spawner, peripherals: CanPeripherals) {
    let can = CanController::new_can2(
        peripherals.can2,
        peripherals.pb12,
        peripherals.pb13,
        500_000,
        peripherals.can1,
        peripherals.pa11,
        peripherals.pa12,
    ).await;

    let can_mutex = Mutex::<CriticalSectionRawMutex, _>::new(can);
    static CAN: StaticCell<Mutex<CriticalSectionRawMutex, CanController>> = StaticCell::new();
    let can = CAN.init(can_mutex);

    spawner.spawn(can_reader()).unwrap();
    spawner.spawn(can_writer()).unwrap();
}

#[task]
pub async fn can_sender() {
    let can = CAN.get().unwrap();
    loop {
        let message = CAN_MSG_CHANNEL.receive().await;
        let mut can_lock = can.lock().await;
        let _ = can_lock.write_message(&message).await;
    }
}

#[task]
pub async fn can_reader() {
    let can = CAN.get().unwrap();
    loop {

    }
}#[embassy_executor::task]
async fn write_can(mut tx: CanTx<'static>) {
    loop {
        let (id, mes) = CAN_WRITER.receive().await;
        let message = embassy_stm32::can::Frame::new_standard(id, &mes);
        if let Ok(some) = message {
            tx.write(&some).await;
        }
    }
}

pub async fn asb_check() {
    set_driverless_run_state(DriverlessRunState::ASBCheck);
    engage_brake();

    loop {
        Timer::after(Duration::from_millis(100)).await; 
        if get_driverless_run_state() == DriverlessRunState::ReadyToDrive {
            break;
        }
    }
    
    release_brake();
}


