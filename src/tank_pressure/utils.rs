/// Risoluzione dell'ADC in bit
pub const ADC_RESOLUTION: u32 = 12;
/// Valore massimo che può assumere l'ADC
pub const MAX_ADC_VALUE: f32 = (1 << ADC_RESOLUTION) as f32 - 1.0;
/// Tensione di riferimento dell'ADC
pub const ADC_VOLTAGE_REF: f32 = 3.3;

/// Range di tensione del sensore di pressione
pub const MIN_VOLTAGE: f32 = 0.5;
pub const MAX_VOLTAGE: f32 = 4.5;
/// Pressione massima misurabile (in Bar)
pub const MAX_PRESSURE: f32 = 100.0;

/// Fattore di scala per convertire la tensione in pressione
pub const SCALING_FACTOR: f32 = MAX_PRESSURE / (MAX_VOLTAGE - MIN_VOLTAGE);
/// Soglia sotto la quale la pressione è considerata critica
pub const PRESSURE_THRESHOLD: f32 = 6.0;

/// Converte un valore ADC in tensione (V)
pub fn adc_to_voltage(adc_reading: u16) -> f32 {
    adc_reading as f32 * ADC_VOLTAGE_REF / MAX_ADC_VALUE
}

/// Converte una tensione in pressione (Bar)
pub fn voltage_to_pressure(voltage: f32) -> f32 {
    if voltage < MIN_VOLTAGE {
        0.0 // Se il valore è fuori scala, restituisce 0 Bar
    } else if voltage > MAX_VOLTAGE {
        MAX_PRESSURE // Se è oltre il massimo, restituisce la pressione massima
    } else {
        SCALING_FACTOR * (voltage - MIN_VOLTAGE)
    }
} //TODO: cambiare in gestione errori

/// Verifica se la pressione è critica
pub fn brake_pressure_is_critical(pressure_read: (f32, f32)) -> bool {
    let (pressure1, pressure2) = pressure_read;
    pressure1 < PRESSURE_THRESHOLD || pressure2 < PRESSURE_THRESHOLD
}
