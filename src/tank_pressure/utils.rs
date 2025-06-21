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
pub const MAX_PRESSURE: f32 = 10.0;

/// Fattore di scala per convertire la tensione in pressione
pub const SCALING_FACTOR: f32 = MAX_PRESSURE / (MAX_VOLTAGE - MIN_VOLTAGE);


/// Converte un valore ADC in tensione (V)
pub fn adc_to_voltage(adc_reading: u16) -> f32 {
    (adc_reading as f32 * ADC_VOLTAGE_REF / MAX_ADC_VALUE) * 2.0 // dimezzamento di corrente in hardware
}

/// Converte una tensione in pressione (Bar)
pub fn voltage_to_pressure(voltage: f32) -> f32 {
    if voltage < MIN_VOLTAGE {
        0.0 // Se il valore è fuori scala, restituisce 0 Bar
    }
    else {
        SCALING_FACTOR * (voltage - MIN_VOLTAGE)
    }
} 