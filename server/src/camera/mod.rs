#![allow(dead_code)]

use std::{collections::HashMap, error::Error};

use async_std::net::TcpStream;

use crate::{
    asi::asi_api::*,
    bytes::{AsyncReadExtend, AsyncWriteExtend},
    packet::OpenCameraStatusPacket,
};

#[derive(Debug, Clone)]
pub struct Camera {
    pub id: u32,
    pub info: CameraInfo,
    pub controls: Vec<ControlCaps>,
    pub connected: bool,
}

impl Camera {
    async fn new(id: u32) -> Result<Self, Box<dyn Error>> {
        open_camera(id as i32).await?;
        init_camera(id as i32).await?;

        let info = get_camera_property_by_id(id as i32).await?.into();
        let mut controls = vec![];
        for i in 0..get_num_of_controls(id as i32).await? {
            controls.push(get_control_caps(id as i32, i).await?.into())
        }

        Ok(Self {
            id,
            info,
            controls,
            connected: false,
        })
    }

    async fn close(&mut self) -> Result<(), ASIError> {
        close_camera(self.id as i32).await
    }

    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        tcp.write_u32(self.id).await?;
        self.info.write(tcp).await?;
        tcp.write_u16(self.controls.len() as u16).await?;
        for control in &self.controls {
            control.write(tcp).await?;
        }
        tcp.write_bool(self.connected).await
    }
}

#[derive(Debug, Clone)]
pub struct CameraInfo {
    pub name: String,
    pub camera_id: u32,
    pub max_height: u32,
    pub max_width: u32,
    pub is_color_cam: bool,
    pub bayer_pattern: BayerPattern,
    pub supported_bins: Vec<u32>,
    pub supported_video_format: Vec<ImageType>,
    pub pixel_size: f64,
    pub mechanical_shutter: bool,
    pub st4_port: bool,
    pub is_cooler_cam: bool,
    pub is_usb3_host: bool,
    pub is_usb3_camera: bool,
    pub elec_per_adu: f32,
    pub bit_depth: u32,
    pub is_trigger_cam: bool,
}

impl CameraInfo {
    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        tcp.write_str(&self.name).await?;
        tcp.write_u32(self.camera_id).await?;
        tcp.write_u32(self.max_height).await?;
        tcp.write_u32(self.max_width).await?;
        tcp.write_bool(self.is_color_cam).await?;
        self.bayer_pattern.write(tcp).await?;
        tcp.write_u16(self.supported_bins.len() as u16).await?;
        for bin in &self.supported_bins {
            tcp.write_u32(*bin).await?;
        }
        tcp.write_u16(self.supported_video_format.len() as u16)
            .await?;
        for img_type in &self.supported_video_format {
            img_type.write(tcp).await?;
        }
        tcp.write_f64(self.pixel_size).await?;
        tcp.write_bool(self.mechanical_shutter).await?;
        tcp.write_bool(self.st4_port).await?;
        tcp.write_bool(self.is_color_cam).await?;
        tcp.write_bool(self.is_usb3_host).await?;
        tcp.write_bool(self.is_usb3_camera).await?;
        tcp.write_f32(self.elec_per_adu).await?;
        tcp.write_u32(self.bit_depth).await?;
        tcp.write_bool(self.is_trigger_cam).await
    }
}

impl From<ASICameraInfo> for CameraInfo {
    fn from(value: ASICameraInfo) -> Self {
        Self {
            name: value.name,
            camera_id: value.camera_id as u32,
            max_height: value.max_height as u32,
            max_width: value.max_width as u32,
            is_color_cam: value.is_color_cam,
            bayer_pattern: value.bayer_pattern.into(),
            supported_bins: value.supported_bins.iter().map(|x| *x as u32).collect(),
            supported_video_format: value
                .supported_video_format
                .iter()
                .map(|x| (*x).into())
                .collect(),
            pixel_size: value.pixel_size,
            mechanical_shutter: value.mechanical_shutter,
            st4_port: value.st4_port,
            is_cooler_cam: value.is_cooler_cam,
            is_usb3_host: value.is_usb3_host,
            is_usb3_camera: value.is_usb3_camera,
            elec_per_adu: value.elec_per_adu,
            bit_depth: value.bit_depth as u32,
            is_trigger_cam: value.is_trigger_cam,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BayerPattern {
    RG,
    BG,
    GR,
    GB,
}

impl BayerPattern {
    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        match self {
            BayerPattern::RG => tcp.write_u8(0).await,
            BayerPattern::BG => tcp.write_u8(1).await,
            BayerPattern::GR => tcp.write_u8(2).await,
            BayerPattern::GB => tcp.write_u8(3).await,
        }
    }
}

impl From<ASIBayerPattern> for BayerPattern {
    fn from(value: ASIBayerPattern) -> Self {
        match value {
            ASIBayerPattern::RG => Self::RG,
            ASIBayerPattern::BG => Self::BG,
            ASIBayerPattern::GR => Self::GR,
            ASIBayerPattern::GB => Self::GB,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Raw8,
    Rgb24,
    Raw16,
    Y8,
}

impl ImageType {
    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        match self {
            ImageType::Raw8 => tcp.write_u8(0).await,
            ImageType::Rgb24 => tcp.write_u8(1).await,
            ImageType::Raw16 => tcp.write_u8(2).await,
            ImageType::Y8 => tcp.write_u8(3).await,
        }
    }
}

impl From<ASIImageType> for ImageType {
    fn from(value: ASIImageType) -> Self {
        match value {
            ASIImageType::Raw8 => Self::Raw8,
            ASIImageType::Rgb24 => Self::Rgb24,
            ASIImageType::Raw16 => Self::Raw16,
            ASIImageType::Y8 => Self::Y8,
            ASIImageType::End => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ControlType {
    Gain,
    Exposure,
    Gamma,
    WbR,
    WbB,
    Offset,
    BandwidthOverload,
    OverClock,
    Temperature,
    Flip,
    AutoMaxGain,
    AutoMaxExp,
    AutoTargetBrightness,
    HardwareBin,
    HighSpeedMode,
    CoolerPowerPerc,
    TargetTemp,
    CoolerOn,
    MonoBin,
    FanOn,
    PatternAdjust,
    AntiDewHeater,
}

impl ControlType {
    pub async fn read(tcp: &mut TcpStream) -> std::io::Result<Self> {
        match tcp.read_u8().await? {
            0 => Ok(Self::Gain),
            1 => Ok(Self::Exposure),
            2 => Ok(Self::Gamma),
            3 => Ok(Self::WbR),
            4 => Ok(Self::WbB),
            5 => Ok(Self::Offset),
            6 => Ok(Self::BandwidthOverload),
            7 => Ok(Self::OverClock),
            8 => Ok(Self::Temperature),
            9 => Ok(Self::Flip),
            10 => Ok(Self::AutoMaxGain),
            11 => Ok(Self::AutoMaxExp),
            12 => Ok(Self::AutoTargetBrightness),
            13 => Ok(Self::HardwareBin),
            14 => Ok(Self::HighSpeedMode),
            15 => Ok(Self::CoolerPowerPerc),
            16 => Ok(Self::TargetTemp),
            17 => Ok(Self::CoolerOn),
            18 => Ok(Self::MonoBin),
            19 => Ok(Self::FanOn),
            20 => Ok(Self::PatternAdjust),
            21 => Ok(Self::AntiDewHeater),
            _ => panic!(),
        }
    }

    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        match self {
            ControlType::Gain => tcp.write_u8(0).await,
            ControlType::Exposure => tcp.write_u8(1).await,
            ControlType::Gamma => tcp.write_u8(2).await,
            ControlType::WbR => tcp.write_u8(3).await,
            ControlType::WbB => tcp.write_u8(4).await,
            ControlType::Offset => tcp.write_u8(5).await,
            ControlType::BandwidthOverload => tcp.write_u8(6).await,
            ControlType::OverClock => tcp.write_u8(7).await,
            ControlType::Temperature => tcp.write_u8(8).await,
            ControlType::Flip => tcp.write_u8(9).await,
            ControlType::AutoMaxGain => tcp.write_u8(10).await,
            ControlType::AutoMaxExp => tcp.write_u8(11).await,
            ControlType::AutoTargetBrightness => tcp.write_u8(12).await,
            ControlType::HardwareBin => tcp.write_u8(13).await,
            ControlType::HighSpeedMode => tcp.write_u8(14).await,
            ControlType::CoolerPowerPerc => tcp.write_u8(15).await,
            ControlType::TargetTemp => tcp.write_u8(16).await,
            ControlType::CoolerOn => tcp.write_u8(17).await,
            ControlType::MonoBin => tcp.write_u8(18).await,
            ControlType::FanOn => tcp.write_u8(19).await,
            ControlType::PatternAdjust => tcp.write_u8(20).await,
            ControlType::AntiDewHeater => tcp.write_u8(21).await,
        }
    }
}

impl From<ASIControlType> for ControlType {
    fn from(value: ASIControlType) -> Self {
        match value {
            ASIControlType::Gain => Self::Gain,
            ASIControlType::Exposure => Self::Exposure,
            ASIControlType::Gamma => Self::Gamma,
            ASIControlType::WbR => Self::WbR,
            ASIControlType::WbB => Self::WbB,
            ASIControlType::Offset => Self::Offset,
            ASIControlType::BandwidthOverload => Self::BandwidthOverload,
            ASIControlType::OverClock => Self::OverClock,
            ASIControlType::Temperature => Self::Temperature,
            ASIControlType::Flip => Self::Flip,
            ASIControlType::AutoMaxGain => Self::AutoMaxGain,
            ASIControlType::AutoMaxExp => Self::AutoMaxExp,
            ASIControlType::AutoTargetBrightness => Self::AutoTargetBrightness,
            ASIControlType::HardwareBin => Self::HardwareBin,
            ASIControlType::HighSpeedMode => Self::HighSpeedMode,
            ASIControlType::CoolerPowerPerc => Self::CoolerPowerPerc,
            ASIControlType::TargetTemp => Self::TargetTemp,
            ASIControlType::CoolerOn => Self::CoolerOn,
            ASIControlType::MonoBin => Self::MonoBin,
            ASIControlType::FanOn => Self::FanOn,
            ASIControlType::PatternAdjust => Self::PatternAdjust,
            ASIControlType::AntiDewHeater => Self::AntiDewHeater,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ControlCaps {
    pub name: String,
    pub description: String,
    pub max_value: u32,
    pub min_value: u32,
    pub default_value: u32,
    pub is_auto_supported: bool,
    pub is_writable: bool,
    pub control_type: ControlType,
}

impl ControlCaps {
    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        tcp.write_str(&self.name).await?;
        tcp.write_str(&self.description).await?;
        tcp.write_u32(self.max_value).await?;
        tcp.write_u32(self.min_value).await?;
        tcp.write_u32(self.default_value).await?;
        tcp.write_bool(self.is_auto_supported).await?;
        tcp.write_bool(self.is_writable).await?;
        self.control_type.write(tcp).await
    }
}

impl From<ASIControlCaps> for ControlCaps {
    fn from(value: ASIControlCaps) -> Self {
        Self {
            name: value.name,
            description: value.description,
            max_value: value.max_value as u32,
            min_value: value.min_value as u32,
            default_value: value.default_value as u32,
            is_auto_supported: value.is_auto_supported,
            is_writable: value.is_writable,
            control_type: value.control_type.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CameraManager {
    cams: HashMap<u32, Camera>,
}

impl CameraManager {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let connected_cams = get_num_of_connected_cameras().await;

        let mut cam_hash = HashMap::new();
        for cam_index in 0..connected_cams {
            let cam_info = get_camera_property(cam_index).await?;
            let camera = Camera::new(cam_info.camera_id as u32).await?;
            cam_hash.insert(cam_info.camera_id as u32, camera);
        }

        Ok(Self { cams: cam_hash })
    }

    pub async fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let connected_cams = get_num_of_connected_cameras().await;

        let last_cam_ids = self.cams.keys().copied().collect::<Vec<_>>();

        let mut ids = vec![];
        for cam_index in 0..connected_cams {
            let cam_info = get_camera_property(cam_index).await?;
            if last_cam_ids
                .iter()
                .any(|id| *id == cam_info.camera_id as u32)
            {
                let camera = Camera::new(cam_info.camera_id as u32).await?;
                self.cams.insert(cam_info.camera_id as u32, camera);
            }
            ids.push(cam_info.camera_id as u32);
        }

        for id in last_cam_ids {
            if !ids.iter().any(|cam_id| *cam_id == id) {
                self.cams.remove(&id);
            }
        }

        Ok(())
    }

    pub async fn connected_cams_count(&self) -> usize {
        get_num_of_connected_cameras().await as usize
    }

    pub async fn connected_cams(&mut self) -> Result<Vec<Camera>, Box<dyn Error>> {
        self.update().await?;
        Ok(self.cams.iter().map(|x| x.1.clone()).collect())
    }

    pub fn monopoly_camera(&mut self, id: u32) -> OpenCameraStatusPacket {
        for (cam_id, cam) in self.cams.iter_mut() {
            if *cam_id == id {
                if !cam.connected {
                    return OpenCameraStatusPacket::Success(id);
                } else {
                    return OpenCameraStatusPacket::CameraInUse;
                }
            }
        }
        OpenCameraStatusPacket::NoCameraFound
    }
}
