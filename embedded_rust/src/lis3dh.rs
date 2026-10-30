use embedded_hal::blocking::i2c::{Write, WriteRead};

pub struct Lis3dh<I2C> {
    i2c: I2C,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<I2C, E> Lis3dh<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    const ADDRESS: u8 = 0x18;

    const CTRL_REG_1: u8 = 0x20;
    const CTRL_REG_4: u8 = 0x23;

    const OUT_X_L: u8 = 0x28;
    const OUT_Y_L: u8 = 0x2A;
    const OUT_Z_L: u8 = 0x2C;

    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn init(&mut self) -> Result<(), E> {
        self.write_reg(Self::CTRL_REG_1, 0x97)?;
        self.write_reg(Self::CTRL_REG_4, 0x80)?;

        Ok(())
    }

    pub fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), E> {
        self.i2c.write(Self::ADDRESS, &[reg, val])
    }

    pub fn read_reg(&mut self, reg: u8) -> Result<u8, E> {
        let mut val = [0u8; 1];
        self.i2c.write_read(Self::ADDRESS, &[reg], &mut val)?;
        Ok(val[0])
    }

    pub fn update(&mut self) -> Result<(), E> {
        self.x = self.read_axis(Self::OUT_X_L)?;
        self.y = self.read_axis(Self::OUT_Y_L)?;
        self.z = self.read_axis(Self::OUT_Z_L)?;
        Ok(())
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }

    fn read_axis(&mut self, low_reg: u8) -> Result<f32, E> {
        let lsb = self.read_reg(low_reg)?;
        let msb = self.read_reg(low_reg | 0x01)?;

        let raw_value = ((msb as u16) << 8) | (lsb as u16);
        Ok(Self::calc_value(raw_value))
    }

    fn calc_value(raw_value: u16) -> f32 {
        let sensitivity: f32 = 0.004;
        let scaling: f32 = 64.0 / sensitivity;
        (raw_value as i16) as f32 / scaling
    }
}