#![allow(dead_code)]

use async_std::net::TcpStream;

use crate::{
    bytes::AsyncReadExtend,
    protocol::camera::{Camera, ControlType},
};

#[derive(Debug)]
pub enum Requests {
    GetConnectedCameras,
    GetControlValue(GetControlValuePacket),
    SetControlValue(SetControlValuePacket),
}

impl Requests {
    pub async fn decode(tcp: &mut TcpStream) -> std::io::Result<Self> {
        let id = tcp.read_u8().await?;
        match id {
            0x0 => Ok(Self::GetConnectedCameras),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Responses {
    ConnectedCameras(ConnectedCamerasPacket),
    ControlValue(ControlValuePacket),
    ASIError(ASIErrorCode),
    None,
}

impl Responses {
    pub async fn encode(&self, _tcp: &mut TcpStream) -> std::io::Result<()> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ConnectedCamerasPacket(Vec<Camera>);

#[derive(Debug, Clone)]
pub struct GetControlValuePacket {
    id: u32,
    control_type: ControlType,
}
#[derive(Debug, Clone)]
pub struct ControlValuePacket {
    id: u32,
    control_type: ControlType,
    value: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct SetControlValuePacket {
    id: u32,
    control_type: ControlType,
    value: i32,
}

#[derive(Debug, Clone)]
pub enum ASIErrorCode {
    InvalidIndex,
    InvalidID,
    InvalidControlType,
    CameraClosed,
    CameraRemoved,
    InvalidPath,
    InvalidFileformat,
    InvalidSize,
    InvalidImgtype,
    OutofBoundary,
    Timeout,
    InvalidSequence,
    BufferTooSmall,
    VideoModeActive,
    ExposureInProgress,
    GeneralError,
    InvalidMode,
}
