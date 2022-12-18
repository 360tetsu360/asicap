use async_std::{io::ReadExt, net::TcpStream};
use async_trait::async_trait;

#[async_trait]
pub trait AsyncReadExtend {
    async fn read_bool(&mut self) -> std::io::Result<bool>;
    async fn read_u8(&mut self) -> std::io::Result<u8>;
    async fn read_i32(&mut self) -> std::io::Result<i32>;
    async fn read_u32(&mut self) -> std::io::Result<u32>;
    async fn read_f32(&mut self) -> std::io::Result<f32>;
    async fn read_f64(&mut self) -> std::io::Result<f64>;
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
}
