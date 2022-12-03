#![allow(dead_code)]

use async_std::net::TcpStream;

pub enum Request {}

impl Request {
    pub async fn decode(_tcp: &mut TcpStream) -> std::io::Result<Self> {
        todo!()
    }
}

pub enum Response {}

impl Response {
    pub async fn encode(&self, _tcp: &mut TcpStream) -> std::io::Result<()> {
        todo!()
    }
}
