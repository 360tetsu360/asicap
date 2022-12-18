#![allow(dead_code)]

use async_std::net::TcpStream;

use crate::bytes::AsyncWriteExtend;

#[derive(Debug, Clone)]
pub struct Camera {
    pub id: u32,
    pub info: CameraInfo,
    pub controls: Vec<ControlCaps>,
}

impl Camera {
    pub async fn write(&self, tcp: &mut TcpStream) -> std::io::Result<()> {
        tcp.write_u32(self.id).await?;
        self.info.write(tcp).await?;
        tcp.write_u16(self.controls.len() as u16).await?;
        for control in &self.controls {
            control.write(tcp).await?;
        }
        Ok(())
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
