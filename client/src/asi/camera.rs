use async_std::net::TcpStream;

use crate::bytes::{AsyncReadExtend, AsyncWriteExtend};

#[derive(Debug, Clone)]
pub struct Camera {
    pub id: u32,
    pub info: CameraInfo,
    pub controls: Vec<ControlCaps>,
    pub connected: bool,
}

impl Camera {
    pub async fn read(tcp: &mut TcpStream) -> std::io::Result<Self> {
        Ok(Self {
            id: tcp.read_u32().await?,
            info: CameraInfo::read(tcp).await?,
            controls: {
                let len = tcp.read_u16().await?;
                let mut ret = vec![];
                for _ in 0..len {
                    ret.push(ControlCaps::read(tcp).await?);
                }
                ret
            },
            connected: tcp.read_bool().await?,
        })
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
    pub async fn read(tcp: &mut TcpStream) -> std::io::Result<Self> {
        Ok(Self {
            name: tcp.read_string().await?,
            camera_id: tcp.read_u32().await?,
            max_height: tcp.read_u32().await?,
            max_width: tcp.read_u32().await?,
            is_color_cam: tcp.read_bool().await?,
            bayer_pattern: BayerPattern::read(tcp).await?,
            supported_bins: {
                let len = tcp.read_u16().await?;
                let mut ret = vec![];
                for _ in 0..len {
                    ret.push(tcp.read_u32().await?);
                }
                ret
            },
            supported_video_format: {
                let len = tcp.read_u16().await?;
                let mut ret = vec![];
                for _ in 0..len {
                    ret.push(ImageType::read(tcp).await?)
                }
                ret
            },
            pixel_size: tcp.read_f64().await?,
            mechanical_shutter: tcp.read_bool().await?,
            st4_port: tcp.read_bool().await?,
            is_cooler_cam: tcp.read_bool().await?,
            is_usb3_host: tcp.read_bool().await?,
            is_usb3_camera: tcp.read_bool().await?,
            elec_per_adu: tcp.read_f32().await?,
            bit_depth: tcp.read_u32().await?,
            is_trigger_cam: tcp.read_bool().await?,
        })
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
    pub async fn read(tcp: &mut TcpStream) -> std::io::Result<Self> {
        match tcp.read_u8().await? {
            0 => Ok(Self::RG),
            1 => Ok(Self::BG),
            2 => Ok(Self::GR),
            3 => Ok(Self::GB),
            _ => panic!(),
        }
    }
}

impl ToString for BayerPattern {
    fn to_string(&self) -> String {
        match self {
            BayerPattern::RG => "RG".to_string(),
            BayerPattern::BG => "BG".to_string(),
            BayerPattern::GR => "GR".to_string(),
            BayerPattern::GB => "GB".to_string(),
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
    pub async fn read(tcp: &mut TcpStream) -> std::io::Result<Self> {
        match tcp.read_u8().await? {
            0 => Ok(Self::Raw8),
            1 => Ok(Self::Rgb24),
            2 => Ok(Self::Raw16),
            3 => Ok(Self::Y8),
            _ => panic!(),
        }
    }
}

impl ToString for ImageType {
    fn to_string(&self) -> String {
        match self {
            ImageType::Raw8 => "RAW8".to_string(),
            ImageType::Rgb24 => "RGB24".to_string(),
            ImageType::Raw16 => "RAW16".to_string(),
            ImageType::Y8 => "Y8".to_string(),
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

impl ToString for ControlType {
    fn to_string(&self) -> String {
        match self {
            ControlType::Gain => "Gain".to_string(),
            ControlType::Exposure => "Exposure".to_string(),
            ControlType::Gamma => "Gamma".to_string(),
            ControlType::WbR => "WbR".to_string(),
            ControlType::WbB => "WbB".to_string(),
            ControlType::Offset => "Offset".to_string(),
            ControlType::BandwidthOverload => "BandwidthOverload".to_string(),
            ControlType::OverClock => "OverClock".to_string(),
            ControlType::Temperature => "Temperature".to_string(),
            ControlType::Flip => "Flip".to_string(),
            ControlType::AutoMaxGain => "AutoMaxGain".to_string(),
            ControlType::AutoMaxExp => "AutoMaxExp".to_string(),
            ControlType::AutoTargetBrightness => "AutoTargetBrightness".to_string(),
            ControlType::HardwareBin => "HardwareBin".to_string(),
            ControlType::HighSpeedMode => "HighSpeedMode".to_string(),
            ControlType::CoolerPowerPerc => "CoolerPowerPerc".to_string(),
            ControlType::TargetTemp => "TargetTemp".to_string(),
            ControlType::CoolerOn => "CoolerOn".to_string(),
            ControlType::MonoBin => "MonoBin".to_string(),
            ControlType::FanOn => "FanOn".to_string(),
            ControlType::PatternAdjust => "PatternAdjust".to_string(),
            ControlType::AntiDewHeater => "AntiDewHeater".to_string(),
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
    pub async fn read(tcp: &mut TcpStream) -> std::io::Result<Self> {
        Ok(Self {
            name: tcp.read_string().await?,
            description: tcp.read_string().await?,
            max_value: tcp.read_u32().await?,
            min_value: tcp.read_u32().await?,
            default_value: tcp.read_u32().await?,
            is_auto_supported: tcp.read_bool().await?,
            is_writable: tcp.read_bool().await?,
            control_type: ControlType::read(tcp).await?,
        })
    }
}
