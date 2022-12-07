#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Camera {
    pub id: i32,
    pub info: CameraInfo,
    pub controls: Vec<ControlCaps>,
}

#[derive(Debug, Clone)]
pub struct CameraInfo {
    pub name: String,
    pub camera_id: i32,
    pub max_height: i32,
    pub max_width: i32,
    pub is_color_cam: bool,
    pub bayer_pattern: BayerPattern,
    pub supported_bins: [i32; 16],
    pub supported_video_format: Vec<ImageType>,
    pub pixel_size: f64,
    pub mechanical_shutter: bool,
    pub st4_port: bool,
    pub is_cooler_cam: bool,
    pub is_usb3_host: bool,
    pub is_usb3_camera: bool,
    pub elec_per_adu: f32,
    pub bit_depth: i32,
    pub is_trigger_cam: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum BayerPattern {
    RG,
    BG,
    GR,
    GB,
}

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Raw8,
    Rgb24,
    Raw16,
    Y8,
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

#[derive(Debug, Clone)]
pub struct ControlCaps {
    pub name: String,
    pub description: String,
    pub max_value: i32,
    pub min_value: i32,
    pub default_value: i32,
    pub is_auto_supported: bool,
    pub is_writable: bool,
    pub control_type: ControlType,
}
