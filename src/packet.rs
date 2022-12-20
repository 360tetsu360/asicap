#![allow(dead_code)]

use async_std::net::TcpStream;

use crate::{
    bytes::{AsyncReadExtend, AsyncWriteExtend},
    camera::{Camera, ControlType},
};

#[derive(Debug)]
pub enum Requests {
    GetConnectedCameras,                    // 0x0
    GetControlValue(GetControlValuePacket), // 0x1
    SetControlValue(SetControlValuePacket), // 0x2
    OpenCamera(u32),                        // 0x3
}

impl Requests {
    pub async fn decode(tcp: &mut TcpStream) -> std::io::Result<Self> {
        let id = tcp.read_u8().await?;
        match id {
            0x0 => Ok(Self::GetConnectedCameras),
            0x3 => Ok(Self::OpenCamera(tcp.read_u32().await?)),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Responses {
    ConnectedCameras(ConnectedCamerasPacket), // 0x0
    ControlValue(ControlValuePacket),         // 0x1
    OpenCameraStatus(OpenCameraStatusPacket), // 0x2
    ASIError(ASIErrorCode),                   // 0x3
    None,                                     // 0x4
}

impl Responses {
    pub async fn encode(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        match self {
            Responses::ConnectedCameras(packet) => {
                tcp.write_u8(0x0).await?;
                packet.write(tcp).await
            },
            Responses::ControlValue(_) => todo!(),
            Responses::OpenCameraStatus(status) => {
                tcp.write_u8(0x2).await?;
                status.write(tcp).await
            },
            Responses::ASIError(_) => todo!(),
            Responses::None => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectedCamerasPacket(pub Vec<Camera>);

impl ConnectedCamerasPacket {
    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        tcp.write_u16(self.0.len() as u16).await?;
        for camera in &self.0 {
            camera.write(tcp).await?;
        }
        Ok(())
    }
}

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

#[derive(Debug, Clone, Copy)]
pub enum OpenCameraStatusPacket {
    Success,
    NoCameraFound,
    CameraInUse,
}

impl OpenCameraStatusPacket {
    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        match self {
            OpenCameraStatusPacket::Success => tcp.write_u8(0x0).await,
            OpenCameraStatusPacket::NoCameraFound => tcp.write_u8(0x1).await,
            OpenCameraStatusPacket::CameraInUse => tcp.write_u8(0x2).await,
        }
    }
}
