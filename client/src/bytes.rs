use async_std::{
    io::{ReadExt, WriteExt},
    net::TcpStream,
};
use async_trait::async_trait;

#[async_trait]
pub trait AsyncReadExtend {
    async fn read_bool(&mut self) -> std::io::Result<bool>;
    async fn read_u8(&mut self) -> std::io::Result<u8>;
    async fn read_u16(&mut self) -> std::io::Result<u16>;
    async fn read_i32(&mut self) -> std::io::Result<i32>;
    async fn read_u32(&mut self) -> std::io::Result<u32>;
    async fn read_f32(&mut self) -> std::io::Result<f32>;
    async fn read_f64(&mut self) -> std::io::Result<f64>;
    async fn read_string(&mut self) -> std::io::Result<String>;
}

#[async_trait]
impl AsyncReadExtend for TcpStream {
    async fn read_bool(&mut self) -> std::io::Result<bool> {
        let mut b = [0u8; 1];
        self.read_exact(&mut b).await?;
        return Ok(b[0] != 0);
    }

    async fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut b = [0u8; 1];
        self.read_exact(&mut b).await?;
        Ok(b[0])
    }

    async fn read_u16(&mut self) -> std::io::Result<u16> {
        let mut b = [0u8; 2];
        self.read_exact(&mut b).await?;
        Ok(u16::from_be_bytes(b))
    }

    async fn read_i32(&mut self) -> std::io::Result<i32> {
        let mut b = [0u8; 4];
        self.read_exact(&mut b).await?;
        Ok(i32::from_be_bytes(b))
    }

    async fn read_u32(&mut self) -> std::io::Result<u32> {
        let mut b = [0u8; 4];
        self.read_exact(&mut b).await?;
        Ok(u32::from_be_bytes(b))
    }

    async fn read_f32(&mut self) -> std::io::Result<f32> {
        let mut b = [0u8; 4];
        self.read_exact(&mut b).await?;
        Ok(f32::from_be_bytes(b))
    }

    async fn read_f64(&mut self) -> std::io::Result<f64> {
        let mut b = [0u8; 8];
        self.read_exact(&mut b).await?;
        Ok(f64::from_be_bytes(b))
    }

    async fn read_string(&mut self) -> std::io::Result<String> {
        let strlen = self.read_u16().await?;
        let mut string_buf = vec![0u8; strlen as usize];
        self.read_exact(&mut string_buf).await?;
        String::from_utf8(string_buf)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))
    }
}

#[async_trait]
pub trait AsyncWriteExtend {
    async fn write_bool(&mut self, v: bool) -> std::io::Result<()>;
    async fn write_u8(&mut self, v: u8) -> std::io::Result<()>;
    async fn write_i32(&mut self, v: i32) -> std::io::Result<()>;
    async fn write_u32(&mut self, v: u32) -> std::io::Result<()>;
    async fn write_f32(&mut self, v: f32) -> std::io::Result<()>;
    async fn write_f64(&mut self, v: f64) -> std::io::Result<()>;
}

#[async_trait]
impl AsyncWriteExtend for TcpStream {
    async fn write_bool(&mut self, v: bool) -> std::io::Result<()> {
        self.write_all(&[v as u8]).await
    }

    async fn write_u8(&mut self, v: u8) -> std::io::Result<()> {
        self.write_all(&[v]).await
    }

    async fn write_i32(&mut self, v: i32) -> std::io::Result<()> {
        self.write_all(&v.to_be_bytes()).await
    }

    async fn write_u32(&mut self, v: u32) -> std::io::Result<()> {
        self.write_all(&v.to_be_bytes()).await
    }

    async fn write_f32(&mut self, v: f32) -> std::io::Result<()> {
        self.write_all(&v.to_be_bytes()).await
    }

    async fn write_f64(&mut self, v: f64) -> std::io::Result<()> {
        self.write_all(&v.to_be_bytes()).await
    }
}
