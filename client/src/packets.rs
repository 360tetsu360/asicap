use async_std::{io::WriteExt, net::TcpStream};

use crate::{
    asi::camera::{Camera, ControlType},
    bytes::{AsyncReadExtend, AsyncWriteExtend},
};

#[derive(Debug)]
pub enum Requests {
    GetConnectedCameras,                    // 0x0
    GetControlValue(GetControlValuePacket), // 0x1
    SetControlValue(SetControlValuePacket), // 0x2
    OpenCamera(u32),                        // 0x3
}

impl Requests {
    pub async fn encode(&self, tcp: &mut TcpStream, id: u32) -> std::io::Result<()> {
        tcp.write_all(&[0xA5]).await?;
        tcp.write_all(&id.to_be_bytes()).await?;

        match self {
            Requests::GetConnectedCameras => {
                tcp.write_u8(0x0).await?;
            }
            Requests::GetControlValue(_) => todo!(),
            Requests::SetControlValue(_) => todo!(),
            Requests::OpenCamera(id) => {
                tcp.write_u8(0x3).await?;
                tcp.write_u32(*id).await?
            }
        }
        Ok(())
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
    pub async fn decode(tcp: &mut TcpStream) -> std::io::Result<Self> {
        match tcp.read_u8().await? {
            0x0 => Ok(Self::ConnectedCameras(
                ConnectedCamerasPacket::read(tcp).await?,
            )),
            0x1 => todo!(),
            0x2 => Ok(Self::OpenCameraStatus(
                OpenCameraStatusPacket::read(tcp).await?,
            )),
            0x3 => todo!(),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectedCamerasPacket(pub Vec<Camera>);

impl ConnectedCamerasPacket {
    pub async fn read(tcp: &mut TcpStream) -> std::io::Result<Self> {
        let len = tcp.read_u16().await?;
        let mut cams = vec![];
        for _ in 0..len {
            cams.push(Camera::read(tcp).await?);
        }
        Ok(Self(cams))
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
    pub async fn read(tcp: &mut TcpStream) -> std::io::Result<Self> {
        match tcp.read_u8().await? {
            0x0 => Ok(Self::Success),
            0x1 => Ok(Self::NoCameraFound),
            0x2 => Ok(Self::CameraInUse),
            _ => panic!(),
        }
    }
}
