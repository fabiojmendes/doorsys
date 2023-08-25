use esp_idf_hal::gpio::{Output, OutputPin, PinDriver};

pub struct Door<'d, T: OutputPin> {
    driver: PinDriver<'d, T, Output>,
}

impl<T: OutputPin> Door<'_, T> {
    pub fn new(pin: T) -> anyhow::Result<Self> {
        let mut driver = PinDriver::output_od(pin)?;
        driver.set_high()?;

        Ok(Door { driver })
    }

    pub fn open(&mut self) -> anyhow::Result<()> {
        self.driver.set_low()?;
        Ok(())
    }

    pub fn close(&mut self) -> anyhow::Result<()> {
        self.driver.set_high()?;
        Ok(())
    }
}
