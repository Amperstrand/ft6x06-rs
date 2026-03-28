use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::{
    common::{
        self, CTRL_ADDR, GEST_ID_ADDR, G_MODE_ADDR, PERIOD_ACTIVE_ADDR, TIME_ENTER_MONITOR_ADDR,
    },
    ControlMode, DriverError, InterruptMode, TouchEvent,
};

const DEFAULT_I2C_ADDR: u8 = 0x38;

pub struct FT6x06<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C: I2c<SevenBitAddress>> FT6x06<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            addr: DEFAULT_I2C_ADDR,
        }
    }

    pub fn new_with_addr(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }

    pub fn get_touch_event(&mut self) -> Result<Option<TouchEvent>, DriverError<I2C::Error>> {
        let mut buf = [0u8; 14];
        self.i2c.write_read(self.addr, &[GEST_ID_ADDR], &mut buf)?;
        Ok(common::touch_event_from_buf(&buf))
    }

    pub fn td_status(&mut self) -> Result<u8, DriverError<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.addr, &[0x02], &mut buf)?;
        Ok(buf[0] & 0x0F)
    }

    pub fn chip_id(&mut self) -> Result<u8, DriverError<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.addr, &[0xA8], &mut buf)?;
        Ok(buf[0])
    }

    pub fn firmware_id(&mut self) -> Result<u8, DriverError<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.addr, &[0xA6], &mut buf)?;
        Ok(buf[0])
    }

    pub fn get_control_mode(&mut self) -> Result<ControlMode, DriverError<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.addr, &[CTRL_ADDR], &mut buf)?;
        let value = ControlMode::try_from(buf[0]).map_err(|()| DriverError::InvalidResponse)?;
        Ok(value)
    }

    pub fn set_control_mode(&mut self, mode: ControlMode) -> Result<(), DriverError<I2C::Error>> {
        Ok(self.i2c.write(self.addr, &[CTRL_ADDR, mode as u8])?)
    }

    pub fn get_active_idle_timeout(&mut self) -> Result<u8, DriverError<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(self.addr, &[TIME_ENTER_MONITOR_ADDR], &mut buf)?;
        Ok(buf[0])
    }

    pub fn set_active_idle_timeout(&mut self, timeout: u8) -> Result<(), DriverError<I2C::Error>> {
        Ok(self
            .i2c
            .write(self.addr, &[TIME_ENTER_MONITOR_ADDR, timeout])?)
    }

    pub fn get_report_rates(&mut self) -> Result<(u8, u8), DriverError<I2C::Error>> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.addr, &[PERIOD_ACTIVE_ADDR], &mut buf)?;
        Ok((buf[0], buf[1]))
    }

    pub fn set_report_rates(
        &mut self,
        active_rate: u8,
        monitor_rate: u8,
    ) -> Result<(), DriverError<I2C::Error>> {
        Ok(self
            .i2c
            .write(self.addr, &[PERIOD_ACTIVE_ADDR, active_rate, monitor_rate])?)
    }

    pub fn get_interrupt_mode(&mut self) -> Result<InterruptMode, DriverError<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.addr, &[G_MODE_ADDR], &mut buf)?;
        let value = InterruptMode::try_from(buf[0]).map_err(|()| DriverError::InvalidResponse)?;
        Ok(value)
    }

    pub fn set_interrupt_mode(
        &mut self,
        mode: InterruptMode,
    ) -> Result<(), DriverError<I2C::Error>> {
        Ok(self.i2c.write(self.addr, &[G_MODE_ADDR, mode as u8])?)
    }

    pub fn read_register(&mut self, reg: u8) -> Result<u8, DriverError<I2C::Error>> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(self.addr, &[reg], &mut buf)?;
        Ok(buf[0])
    }

    pub unsafe fn write_register(
        &mut self,
        reg: u8,
        val: u8,
    ) -> Result<(), DriverError<I2C::Error>> {
        Ok(self.i2c.write(self.addr, &[reg, val])?)
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
