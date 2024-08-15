use crate::{chip::{offsets::{FATAL_ERR_INT_CONTROL, OMC0_D4}, Omc0D4}, dma::Dma, error::Error};

impl dyn Dma {
    pub fn init_(&self) {
    }
    pub fn enable_thermal_warning_interrupt(&mut self) -> Result<(), Error> {
        let mut reg = Omc0D4::new(self.read_32(OMC0_D4)?);
        reg.set_thm_warn_en(1);
        self.write_32(OMC0_D4, reg.read())?;

        Ok(())
    }
}
