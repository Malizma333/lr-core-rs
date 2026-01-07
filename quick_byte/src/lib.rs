use std::io::{self, Read, Write};

pub trait QuickRead {
    fn read_u8(&mut self) -> io::Result<u8>;

    fn read_u16_le(&mut self) -> io::Result<u16>;

    fn read_u16_be(&mut self) -> io::Result<u16>;

    fn read_u32_le(&mut self) -> io::Result<u32>;

    fn read_u32_be(&mut self) -> io::Result<u32>;

    fn read_u64_le(&mut self) -> io::Result<u64>;

    fn read_u64_be(&mut self) -> io::Result<u64>;

    fn read_i8(&mut self) -> io::Result<i8>;

    fn read_i16_le(&mut self) -> io::Result<i16>;

    fn read_i16_be(&mut self) -> io::Result<i16>;

    fn read_i32_le(&mut self) -> io::Result<i32>;

    fn read_i32_be(&mut self) -> io::Result<i32>;

    fn read_i64_le(&mut self) -> io::Result<i64>;

    fn read_i64_be(&mut self) -> io::Result<i64>;

    fn read_f32_le(&mut self) -> io::Result<f32>;

    fn read_f32_be(&mut self) -> io::Result<f32>;

    fn read_f64_le(&mut self) -> io::Result<f64>;

    fn read_f64_be(&mut self) -> io::Result<f64>;
}

impl<T: Read> QuickRead for T {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0u8];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16_le(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn read_u16_be(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_u32_be(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_u64_le(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    fn read_u64_be(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    fn read_i8(&mut self) -> io::Result<i8> {
        let mut buf = [0u8];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    fn read_i16_le(&mut self) -> io::Result<i16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }

    fn read_i16_be(&mut self) -> io::Result<i16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    fn read_i32_le(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    fn read_i32_be(&mut self) -> io::Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_i64_le(&mut self) -> io::Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }

    fn read_i64_be(&mut self) -> io::Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    fn read_f32_le(&mut self) -> io::Result<f32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    fn read_f32_be(&mut self) -> io::Result<f32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    fn read_f64_le(&mut self) -> io::Result<f64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }

    fn read_f64_be(&mut self) -> io::Result<f64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }
}

pub trait QuickWrite {
    fn write_u8(&mut self, value: u8) -> io::Result<()>;

    fn write_u16_le(&mut self, value: u16) -> io::Result<()>;

    fn write_u16_be(&mut self, value: u16) -> io::Result<()>;

    fn write_u32_le(&mut self, value: u32) -> io::Result<()>;

    fn write_u32_be(&mut self, value: u32) -> io::Result<()>;

    fn write_u64_le(&mut self, value: u64) -> io::Result<()>;

    fn write_u64_be(&mut self, value: u64) -> io::Result<()>;

    fn write_i8(&mut self, value: i8) -> io::Result<()>;

    fn write_i16_le(&mut self, value: i16) -> io::Result<()>;

    fn write_i16_be(&mut self, value: i16) -> io::Result<()>;

    fn write_i32_le(&mut self, value: i32) -> io::Result<()>;

    fn write_i32_be(&mut self, value: i32) -> io::Result<()>;

    fn write_i64_le(&mut self, value: i64) -> io::Result<()>;

    fn write_i64_be(&mut self, value: i64) -> io::Result<()>;

    fn write_f32_le(&mut self, value: f32) -> io::Result<()>;

    fn write_f32_be(&mut self, value: f32) -> io::Result<()>;

    fn write_f64_le(&mut self, value: f64) -> io::Result<()>;

    fn write_f64_be(&mut self, value: f64) -> io::Result<()>;
}

impl<T: Write> QuickWrite for T {
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        let buf = [value];
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_u16_le(&mut self, value: u16) -> io::Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_u16_be(&mut self, value: u16) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_u32_le(&mut self, value: u32) -> io::Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_u32_be(&mut self, value: u32) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_u64_le(&mut self, value: u64) -> io::Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_u64_be(&mut self, value: u64) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_i8(&mut self, value: i8) -> io::Result<()> {
        let buf = [value as u8];
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_i16_le(&mut self, value: i16) -> io::Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_i16_be(&mut self, value: i16) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_i32_le(&mut self, value: i32) -> io::Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_i32_be(&mut self, value: i32) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_i64_le(&mut self, value: i64) -> io::Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_i64_be(&mut self, value: i64) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_f32_le(&mut self, value: f32) -> io::Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_f32_be(&mut self, value: f32) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_f64_le(&mut self, value: f64) -> io::Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)?;
        Ok(())
    }

    fn write_f64_be(&mut self, value: f64) -> io::Result<()> {
        let buf = value.to_be_bytes();
        self.write_all(&buf)?;
        Ok(())
    }
}
