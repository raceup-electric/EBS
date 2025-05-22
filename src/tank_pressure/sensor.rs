use embassy_stm32::adc::Adc;

pub struct Sensor<ADC, PIN>
where
    ADC: embassy_stm32::adc::Instance,
    PIN: embassy_stm32::adc::AdcChannel<ADC>,
{
    adc: Adc<'static, ADC>,
    pin: PIN,
}

impl<ADC, PIN> Sensor<ADC, PIN>
where
    ADC: embassy_stm32::adc::Instance,
    PIN: embassy_stm32::adc::AdcChannel<ADC>,
{
    pub fn new(adc: Adc<'static, ADC>, pin: PIN) -> Self {
        Self { adc, pin }
    }

    pub fn read(&mut self) -> u16 {
        self.adc.blocking_read(&mut self.pin)
    }
}

