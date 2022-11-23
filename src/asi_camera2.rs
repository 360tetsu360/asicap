#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const ASICAMERA_ID_MAX: u32 = 128;
pub const ASI_BAYER_PATTERN_ASI_BAYER_RG: ASI_BAYER_PATTERN = 0;
pub const ASI_BAYER_PATTERN_ASI_BAYER_BG: ASI_BAYER_PATTERN = 1;
pub const ASI_BAYER_PATTERN_ASI_BAYER_GR: ASI_BAYER_PATTERN = 2;
pub const ASI_BAYER_PATTERN_ASI_BAYER_GB: ASI_BAYER_PATTERN = 3;
pub type ASI_BAYER_PATTERN = ::std::os::raw::c_uint;
pub const ASI_IMG_TYPE_ASI_IMG_RAW8: ASI_IMG_TYPE = 0;
pub const ASI_IMG_TYPE_ASI_IMG_RGB24: ASI_IMG_TYPE = 1;
pub const ASI_IMG_TYPE_ASI_IMG_RAW16: ASI_IMG_TYPE = 2;
pub const ASI_IMG_TYPE_ASI_IMG_Y8: ASI_IMG_TYPE = 3;
pub const ASI_IMG_TYPE_ASI_IMG_END: ASI_IMG_TYPE = -1;
pub type ASI_IMG_TYPE = ::std::os::raw::c_int;
pub const ASI_GUIDE_DIRECTION_ASI_GUIDE_NORTH: ASI_GUIDE_DIRECTION = 0;
pub const ASI_GUIDE_DIRECTION_ASI_GUIDE_SOUTH: ASI_GUIDE_DIRECTION = 1;
pub const ASI_GUIDE_DIRECTION_ASI_GUIDE_EAST: ASI_GUIDE_DIRECTION = 2;
pub const ASI_GUIDE_DIRECTION_ASI_GUIDE_WEST: ASI_GUIDE_DIRECTION = 3;
pub type ASI_GUIDE_DIRECTION = ::std::os::raw::c_uint;
pub const ASI_FLIP_STATUS_ASI_FLIP_NONE: ASI_FLIP_STATUS = 0;
pub const ASI_FLIP_STATUS_ASI_FLIP_HORIZ: ASI_FLIP_STATUS = 1;
pub const ASI_FLIP_STATUS_ASI_FLIP_VERT: ASI_FLIP_STATUS = 2;
pub const ASI_FLIP_STATUS_ASI_FLIP_BOTH: ASI_FLIP_STATUS = 3;
pub type ASI_FLIP_STATUS = ::std::os::raw::c_uint;
pub const ASI_CAMERA_MODE_ASI_MODE_NORMAL: ASI_CAMERA_MODE = 0;
pub const ASI_CAMERA_MODE_ASI_MODE_TRIG_SOFT_EDGE: ASI_CAMERA_MODE = 1;
pub const ASI_CAMERA_MODE_ASI_MODE_TRIG_RISE_EDGE: ASI_CAMERA_MODE = 2;
pub const ASI_CAMERA_MODE_ASI_MODE_TRIG_FALL_EDGE: ASI_CAMERA_MODE = 3;
pub const ASI_CAMERA_MODE_ASI_MODE_TRIG_SOFT_LEVEL: ASI_CAMERA_MODE = 4;
pub const ASI_CAMERA_MODE_ASI_MODE_TRIG_HIGH_LEVEL: ASI_CAMERA_MODE = 5;
pub const ASI_CAMERA_MODE_ASI_MODE_TRIG_LOW_LEVEL: ASI_CAMERA_MODE = 6;
pub const ASI_CAMERA_MODE_ASI_MODE_END: ASI_CAMERA_MODE = -1;
pub type ASI_CAMERA_MODE = ::std::os::raw::c_int;
pub const ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_PINA: ASI_TRIG_OUTPUT = 0;
pub const ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_PINB: ASI_TRIG_OUTPUT = 1;
pub const ASI_TRIG_OUTPUT_ASI_TRIG_OUTPUT_NONE: ASI_TRIG_OUTPUT = -1;
pub type ASI_TRIG_OUTPUT = ::std::os::raw::c_int;
pub use self::ASI_TRIG_OUTPUT as ASI_TRIG_OUTPUT_PIN;
pub const ASI_ERROR_CODE_ASI_SUCCESS: ASI_ERROR_CODE = 0;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_INDEX: ASI_ERROR_CODE = 1;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_ID: ASI_ERROR_CODE = 2;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_CONTROL_TYPE: ASI_ERROR_CODE = 3;
pub const ASI_ERROR_CODE_ASI_ERROR_CAMERA_CLOSED: ASI_ERROR_CODE = 4;
pub const ASI_ERROR_CODE_ASI_ERROR_CAMERA_REMOVED: ASI_ERROR_CODE = 5;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_PATH: ASI_ERROR_CODE = 6;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_FILEFORMAT: ASI_ERROR_CODE = 7;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_SIZE: ASI_ERROR_CODE = 8;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_IMGTYPE: ASI_ERROR_CODE = 9;
pub const ASI_ERROR_CODE_ASI_ERROR_OUTOF_BOUNDARY: ASI_ERROR_CODE = 10;
pub const ASI_ERROR_CODE_ASI_ERROR_TIMEOUT: ASI_ERROR_CODE = 11;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_SEQUENCE: ASI_ERROR_CODE = 12;
pub const ASI_ERROR_CODE_ASI_ERROR_BUFFER_TOO_SMALL: ASI_ERROR_CODE = 13;
pub const ASI_ERROR_CODE_ASI_ERROR_VIDEO_MODE_ACTIVE: ASI_ERROR_CODE = 14;
pub const ASI_ERROR_CODE_ASI_ERROR_EXPOSURE_IN_PROGRESS: ASI_ERROR_CODE = 15;
pub const ASI_ERROR_CODE_ASI_ERROR_GENERAL_ERROR: ASI_ERROR_CODE = 16;
pub const ASI_ERROR_CODE_ASI_ERROR_INVALID_MODE: ASI_ERROR_CODE = 17;
pub const ASI_ERROR_CODE_ASI_ERROR_END: ASI_ERROR_CODE = 18;
pub type ASI_ERROR_CODE = ::std::os::raw::c_uint;
pub const ASI_BOOL_ASI_FALSE: ASI_BOOL = 0;
pub const ASI_BOOL_ASI_TRUE: ASI_BOOL = 1;
pub type ASI_BOOL = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ASI_CAMERA_INFO {
    pub Name: [::std::os::raw::c_char; 64usize],
    pub CameraID: ::std::os::raw::c_int,
    pub MaxHeight: ::std::os::raw::c_long,
    pub MaxWidth: ::std::os::raw::c_long,
    pub IsColorCam: ASI_BOOL,
    pub BayerPattern: ASI_BAYER_PATTERN,
    pub SupportedBins: [::std::os::raw::c_int; 16usize],
    pub SupportedVideoFormat: [ASI_IMG_TYPE; 8usize],
    pub PixelSize: f64,
    pub MechanicalShutter: ASI_BOOL,
    pub ST4Port: ASI_BOOL,
    pub IsCoolerCam: ASI_BOOL,
    pub IsUSB3Host: ASI_BOOL,
    pub IsUSB3Camera: ASI_BOOL,
    pub ElecPerADU: f32,
    pub BitDepth: ::std::os::raw::c_int,
    pub IsTriggerCam: ASI_BOOL,
    pub Unused: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout__ASI_CAMERA_INFO() {
    const UNINIT: ::std::mem::MaybeUninit<_ASI_CAMERA_INFO> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_ASI_CAMERA_INFO>(),
        240usize,
        concat!("Size of: ", stringify!(_ASI_CAMERA_INFO))
    );
    assert_eq!(
        ::std::mem::align_of::<_ASI_CAMERA_INFO>(),
        8usize,
        concat!("Alignment of ", stringify!(_ASI_CAMERA_INFO))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(Name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).CameraID) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(CameraID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MaxHeight) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(MaxHeight)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MaxWidth) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(MaxWidth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsColorCam) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsColorCam)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).BayerPattern) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(BayerPattern)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SupportedBins) as usize - ptr as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(SupportedBins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SupportedVideoFormat) as usize - ptr as usize },
        148usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(SupportedVideoFormat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).PixelSize) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(PixelSize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MechanicalShutter) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(MechanicalShutter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ST4Port) as usize - ptr as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(ST4Port)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsCoolerCam) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsCoolerCam)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsUSB3Host) as usize - ptr as usize },
        204usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsUSB3Host)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsUSB3Camera) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsUSB3Camera)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ElecPerADU) as usize - ptr as usize },
        212usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(ElecPerADU)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).BitDepth) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(BitDepth)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsTriggerCam) as usize - ptr as usize },
        220usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(IsTriggerCam)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Unused) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CAMERA_INFO),
            "::",
            stringify!(Unused)
        )
    );
}
pub type ASI_CAMERA_INFO = _ASI_CAMERA_INFO;
pub const ASI_CONTROL_TYPE_ASI_GAIN: ASI_CONTROL_TYPE = 0;
pub const ASI_CONTROL_TYPE_ASI_EXPOSURE: ASI_CONTROL_TYPE = 1;
pub const ASI_CONTROL_TYPE_ASI_GAMMA: ASI_CONTROL_TYPE = 2;
pub const ASI_CONTROL_TYPE_ASI_WB_R: ASI_CONTROL_TYPE = 3;
pub const ASI_CONTROL_TYPE_ASI_WB_B: ASI_CONTROL_TYPE = 4;
pub const ASI_CONTROL_TYPE_ASI_OFFSET: ASI_CONTROL_TYPE = 5;
pub const ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD: ASI_CONTROL_TYPE = 6;
pub const ASI_CONTROL_TYPE_ASI_OVERCLOCK: ASI_CONTROL_TYPE = 7;
pub const ASI_CONTROL_TYPE_ASI_TEMPERATURE: ASI_CONTROL_TYPE = 8;
pub const ASI_CONTROL_TYPE_ASI_FLIP: ASI_CONTROL_TYPE = 9;
pub const ASI_CONTROL_TYPE_ASI_AUTO_MAX_GAIN: ASI_CONTROL_TYPE = 10;
pub const ASI_CONTROL_TYPE_ASI_AUTO_MAX_EXP: ASI_CONTROL_TYPE = 11;
pub const ASI_CONTROL_TYPE_ASI_AUTO_TARGET_BRIGHTNESS: ASI_CONTROL_TYPE = 12;
pub const ASI_CONTROL_TYPE_ASI_HARDWARE_BIN: ASI_CONTROL_TYPE = 13;
pub const ASI_CONTROL_TYPE_ASI_HIGH_SPEED_MODE: ASI_CONTROL_TYPE = 14;
pub const ASI_CONTROL_TYPE_ASI_COOLER_POWER_PERC: ASI_CONTROL_TYPE = 15;
pub const ASI_CONTROL_TYPE_ASI_TARGET_TEMP: ASI_CONTROL_TYPE = 16;
pub const ASI_CONTROL_TYPE_ASI_COOLER_ON: ASI_CONTROL_TYPE = 17;
pub const ASI_CONTROL_TYPE_ASI_MONO_BIN: ASI_CONTROL_TYPE = 18;
pub const ASI_CONTROL_TYPE_ASI_FAN_ON: ASI_CONTROL_TYPE = 19;
pub const ASI_CONTROL_TYPE_ASI_PATTERN_ADJUST: ASI_CONTROL_TYPE = 20;
pub const ASI_CONTROL_TYPE_ASI_ANTI_DEW_HEATER: ASI_CONTROL_TYPE = 21;
pub type ASI_CONTROL_TYPE = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ASI_CONTROL_CAPS {
    pub Name: [::std::os::raw::c_char; 64usize],
    pub Description: [::std::os::raw::c_char; 128usize],
    pub MaxValue: ::std::os::raw::c_long,
    pub MinValue: ::std::os::raw::c_long,
    pub DefaultValue: ::std::os::raw::c_long,
    pub IsAutoSupported: ASI_BOOL,
    pub IsWritable: ASI_BOOL,
    pub ControlType: ASI_CONTROL_TYPE,
    pub Unused: [::std::os::raw::c_char; 32usize],
}
#[test]
fn bindgen_test_layout__ASI_CONTROL_CAPS() {
    const UNINIT: ::std::mem::MaybeUninit<_ASI_CONTROL_CAPS> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_ASI_CONTROL_CAPS>(),
        248usize,
        concat!("Size of: ", stringify!(_ASI_CONTROL_CAPS))
    );
    assert_eq!(
        ::std::mem::align_of::<_ASI_CONTROL_CAPS>(),
        4usize,
        concat!("Alignment of ", stringify!(_ASI_CONTROL_CAPS))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(Name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Description) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(Description)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MaxValue) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(MaxValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MinValue) as usize - ptr as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(MinValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DefaultValue) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(DefaultValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsAutoSupported) as usize - ptr as usize },
        204usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(IsAutoSupported)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsWritable) as usize - ptr as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(IsWritable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ControlType) as usize - ptr as usize },
        212usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(ControlType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Unused) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_CONTROL_CAPS),
            "::",
            stringify!(Unused)
        )
    );
}
pub type ASI_CONTROL_CAPS = _ASI_CONTROL_CAPS;
pub const ASI_EXPOSURE_STATUS_ASI_EXP_IDLE: ASI_EXPOSURE_STATUS = 0;
pub const ASI_EXPOSURE_STATUS_ASI_EXP_WORKING: ASI_EXPOSURE_STATUS = 1;
pub const ASI_EXPOSURE_STATUS_ASI_EXP_SUCCESS: ASI_EXPOSURE_STATUS = 2;
pub const ASI_EXPOSURE_STATUS_ASI_EXP_FAILED: ASI_EXPOSURE_STATUS = 3;
pub type ASI_EXPOSURE_STATUS = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ASI_ID {
    pub id: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout__ASI_ID() {
    const UNINIT: ::std::mem::MaybeUninit<_ASI_ID> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_ASI_ID>(),
        8usize,
        concat!("Size of: ", stringify!(_ASI_ID))
    );
    assert_eq!(
        ::std::mem::align_of::<_ASI_ID>(),
        1usize,
        concat!("Alignment of ", stringify!(_ASI_ID))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_ID),
            "::",
            stringify!(id)
        )
    );
}
pub type ASI_ID = _ASI_ID;
pub type ASI_SN = ASI_ID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ASI_SUPPORTED_MODE {
    pub SupportedCameraMode: [ASI_CAMERA_MODE; 16usize],
}
#[test]
fn bindgen_test_layout__ASI_SUPPORTED_MODE() {
    const UNINIT: ::std::mem::MaybeUninit<_ASI_SUPPORTED_MODE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_ASI_SUPPORTED_MODE>(),
        64usize,
        concat!("Size of: ", stringify!(_ASI_SUPPORTED_MODE))
    );
    assert_eq!(
        ::std::mem::align_of::<_ASI_SUPPORTED_MODE>(),
        4usize,
        concat!("Alignment of ", stringify!(_ASI_SUPPORTED_MODE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SupportedCameraMode) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ASI_SUPPORTED_MODE),
            "::",
            stringify!(SupportedCameraMode)
        )
    );
}

pub type ASI_SUPPORTED_MODE = _ASI_SUPPORTED_MODE;

#[link(name = "ASICamera2", kind = "static")]
extern "C" {
    #[doc = "Descriptions:"]
    #[doc = "this should be the first API to be called"]
    #[doc = "get number of connected ASI cameras,"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = ""]
    #[doc = "return:number of connected ASI cameras. 1 means 1 camera connected."]
    pub fn ASIGetNumOfConnectedCameras() -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get the product ID of each supported camera, at first set pPIDs as 0 and get length and then malloc a buffer to contain the PIDs"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int* pPIDs: pointer to array of PIDs"]
    #[doc = ""]
    #[doc = "Return: length of the array."]
    #[doc = ""]
    #[doc = "Note: This api will be deprecated. Please use ASICameraCheck instead"]
    pub fn ASIGetProductIDs(pPIDs: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Check if the device is ASI Camera"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int iVID: VID is 0x03C3 for ASI Cameras"]
    #[doc = "int iPID: PID of the device"]
    #[doc = ""]
    #[doc = "Return: ASI_TRUE if the device is ASI Camera, otherwise ASI_FALSE"]
    pub fn ASICameraCheck(
        iVID: ::std::os::raw::c_int,
        iPID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get the property of the connected cameras, you can do this without open the camera."]
    #[doc = "here is the sample code:"]
    #[doc = ""]
    #[doc = "int iNumofConnectCameras = ASIGetNumOfConnectedCameras();"]
    #[doc = "ASI_CAMERA_INFO **ppASICameraInfo = (ASI_CAMERA_INFO **)malloc(sizeof(ASI_CAMERA_INFO *)*iNumofConnectCameras);"]
    #[doc = "for(int i = 0; i < iNumofConnectCameras; i++)"]
    #[doc = "{"]
    #[doc = "ppASICameraInfo[i] = (ASI_CAMERA_INFO *)malloc(sizeof(ASI_CAMERA_INFO ));"]
    #[doc = "ASIGetCameraProperty(ppASICameraInfo[i], i);"]
    #[doc = "}"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "ASI_CAMERA_INFO *pASICameraInfo: Pointer to structure containing the property of camera"]
    #[doc = "user need to malloc the buffer"]
    #[doc = "int iCameraIndex: 0 means the first connect camera, 1 means the second connect camera"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS: Operation is successful"]
    #[doc = "ASI_ERROR_INVALID_INDEX  :no camera connected or index value out of boundary"]
    pub fn ASIGetCameraProperty(
        pASICameraInfo: *mut ASI_CAMERA_INFO,
        iCameraIndex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get the property of the connected cameras by ID."]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "ASI_CAMERA_INFO *pASICameraInfo: Pointer to structure containing the property of camera"]
    #[doc = "user need to malloc the buffer"]
    #[doc = ""]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetCameraPropertyByID(
        iCameraID: ::std::os::raw::c_int,
        pASICameraInfo: *mut ASI_CAMERA_INFO,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "open the camera before any operation to the camera, this will not affect the camera which is capturing"]
    #[doc = "All APIs below need to open the camera at first."]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS: Operation is successful"]
    #[doc = "ASI_ERROR_INVALID_ID  : no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_CAMERA_REMOVED: failed to find the camera, maybe camera has been removed"]
    pub fn ASIOpenCamera(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    #[doc = "Descriptions"]
    #[doc = ""]
    #[doc = "Initialise the camera after open, this function may take some while, this will affect the camera which is capturing"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIInitCamera(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "you need to close the camera to free all the resource"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS :it will return success even the camera already closed"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASICloseCamera(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Get number of controls available for this camera. the camera need be opened at first."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int * piNumberOfControls: pointer to an int to save the number of controls"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetNumOfControls(
        iCameraID: ::std::os::raw::c_int,
        piNumberOfControls: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Get controls property available for this camera. the camera need be opened at first."]
    #[doc = "user need to malloc and maintain the buffer."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int iControlIndex: index of control, NOT control type"]
    #[doc = "ASI_CONTROL_CAPS * pControlCaps: Pointer to structure containing the property of the control"]
    #[doc = "user need to malloc the buffer"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetControlCaps(
        iCameraID: ::std::os::raw::c_int,
        iControlIndex: ::std::os::raw::c_int,
        pControlCaps: *mut ASI_CONTROL_CAPS,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Get controls property value and auto value"]
    #[doc = "note:the value of the temperature is the float value * 10 to convert it to long type, control name is \"Temperature\""]
    #[doc = "because long is the only type for control(except cooler's target temperature, because it is an integer)"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int ControlType: this is get from control property use the API ASIGetControlCaps"]
    #[doc = "long *plValue: pointer to the value you want to save the value get from control"]
    #[doc = "ASI_BOOL *pbAuto: pointer to the ASI_BOOL type"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_INVALID_CONTROL_TYPE, //invalid Control type"]
    pub fn ASIGetControlValue(
        iCameraID: ::std::os::raw::c_int,
        ControlType: ::std::os::raw::c_int,
        plValue: *mut ::std::os::raw::c_long,
        pbAuto: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Set controls property value and auto value"]
    #[doc = "it will return success and set the max value or min value if the value is beyond the boundary"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int ControlType: this is get from control property use the API ASIGetControlCaps"]
    #[doc = "long lValue: the value set to the control"]
    #[doc = "ASI_BOOL bAuto: set the control auto"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_INVALID_CONTROL_TYPE, //invalid Control type"]
    #[doc = "ASI_ERROR_GENERAL_ERROR,//general error, eg: value is out of valid range; operate to camera hareware failed"]
    pub fn ASISetControlValue(
        iCameraID: ::std::os::raw::c_int,
        ControlType: ::std::os::raw::c_int,
        lValue: ::std::os::raw::c_long,
        bAuto: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "set the ROI area before capture."]
    #[doc = "you must stop capture before call it."]
    #[doc = "the width and height is the value after binning."]
    #[doc = "ie. you need to set width to 640 and height to 480 if you want to run at 640X480@BIN2"]
    #[doc = "Specially, ASI120's data size must be times of 1024 which means width*height%1024=0."]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int iWidth,  the width of the ROI area. Make sure iWidth%8 == 0."]
    #[doc = "int iHeight,  the height of the ROI area. Make sure iHeight%2 == 0,"]
    #[doc = "further, for USB2.0 camera ASI120, please make sure that iWidth*iHeight%1024=0."]
    #[doc = "int iBin,   binning method. bin1=1, bin2=2"]
    #[doc = "ASI_IMG_TYPE Img_type: the output format you want"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_INVALID_SIZE, //wrong video format size"]
    #[doc = "ASI_ERROR_INVALID_IMGTYPE, //unsupported image format, make sure iWidth and iHeight and binning is set correct"]
    pub fn ASISetROIFormat(
        iCameraID: ::std::os::raw::c_int,
        iWidth: ::std::os::raw::c_int,
        iHeight: ::std::os::raw::c_int,
        iBin: ::std::os::raw::c_int,
        Img_type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Get the current ROI area setting ."]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int *piWidth,  pointer to the width of the ROI area"]
    #[doc = "int *piHeight, pointer to the height of the ROI area."]
    #[doc = "int *piBin,   pointer to binning method. bin1=1, bin2=2"]
    #[doc = "ASI_IMG_TYPE *pImg_type: pointer to the output format"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetROIFormat(
        iCameraID: ::std::os::raw::c_int,
        piWidth: *mut ::std::os::raw::c_int,
        piHeight: *mut ::std::os::raw::c_int,
        piBin: *mut ::std::os::raw::c_int,
        pImg_type: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Set the start position of the ROI area."]
    #[doc = "you can call this API to move the ROI area when video is streaming"]
    #[doc = "the camera will set the ROI area to the center of the full image as default"]
    #[doc = "at bin2 or bin3 mode, the position is relative to the image after binning"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int iStartX, pointer to the start X"]
    #[doc = "int iStartY  pointer to the start Y"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_OUTOF_BOUNDARY: the start x and start y make the image out of boundary"]
    pub fn ASISetStartPos(
        iCameraID: ::std::os::raw::c_int,
        iStartX: ::std::os::raw::c_int,
        iStartY: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Get the start position of current ROI area ."]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int *piStartX, pointer to the start X"]
    #[doc = "int *piStartY  pointer to the start Y"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetStartPos(
        iCameraID: ::std::os::raw::c_int,
        piStartX: *mut ::std::os::raw::c_int,
        piStartY: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Get the droped frames ."]
    #[doc = "drop frames happen when USB is traffic or harddisk write speed is slow"]
    #[doc = "it will reset to 0 after stop capture"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "int *piDropFrames pointer to drop frames"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetDroppedFrames(
        iCameraID: ::std::os::raw::c_int,
        piDropFrames: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "provide a dark file's path to the function and enable dark subtract"]
    #[doc = "this is used when there is hot pixel or need to do long exposure"]
    #[doc = "you'd better make this dark file from the  \"dark subtract\" funtion"]
    #[doc = "of the \"video capture filter\" directshow page."]
    #[doc = "the dark file's size should be the same of camera's max width and height"]
    #[doc = "and should be RGB8 raw format.it will on even you changed the ROI setting"]
    #[doc = "it only correct the hot pixels if out put isn't 16bit."]
    #[doc = ""]
    #[doc = "it will be remembered in registry. so \"Dark subtract\" is on next time if you close your app."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "char *pcBMPPath: the path to the bmp dark file."]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_PATH, //cannot find the path of the file"]
    #[doc = "ASI_ERROR_INVALID_FILEFORMAT, //the dark file's size should be the same of camera's max width and height"]
    pub fn ASIEnableDarkSubtract(
        iCameraID: ::std::os::raw::c_int,
        pcBMPPath: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Disable the dark subtract function."]
    #[doc = "you'd better call it at start if you don't want to use it."]
    #[doc = "because dark subtract function is remembered on windows platform"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    pub fn ASIDisableDarkSubtract(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Start video capture"]
    #[doc = "then you can get the data from the API ASIGetVideoData"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful, it will return success if already started"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_EXPOSURE_IN_PROGRESS: snap mode is working, you need to stop snap first"]
    pub fn ASIStartVideoCapture(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Stop video capture"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful, it will return success if already stopped"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIStopVideoCapture(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get data from the video buffer.the buffer is very small"]
    #[doc = "you need to call this API as fast as possible, otherwise frame will be discarded"]
    #[doc = "so the best way is maintain one buffer loop and call this API in a loop"]
    #[doc = "please make sure the buffer size is biger enough to hold one image"]
    #[doc = "otherwise the this API will crash"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "unsigned char* pBuffer, caller need to malloc the buffer, make sure the size is big enough"]
    #[doc = "the size in byte:"]
    #[doc = "8bit mono:width*height"]
    #[doc = "16bit mono:width*height*2"]
    #[doc = "RGB24:width*height*3"]
    #[doc = ""]
    #[doc = "int iWaitms, this API will block and wait iWaitms to get one image. the unit is ms"]
    #[doc = "-1 means wait forever. this value is recommend set to exposure*2+500ms"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_TIMEOUT: no image get and timeout"]
    pub fn ASIGetVideoData(
        iCameraID: ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_uchar,
        lBuffSize: ::std::os::raw::c_long,
        iWaitms: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "PulseGuide of the ST4 port on. this function only work on the module which have ST4 port"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_GUIDE_DIRECTION direction the direction of guider"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIPulseGuideOn(
        iCameraID: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "PulseGuide of the ST4 port off. this function only work on the module which have ST4 port"]
    #[doc = "make sure where is ASIPulseGuideOn and there is ASIPulseGuideOff"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_GUIDE_DIRECTION direction the direction of guider"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIPulseGuideOff(
        iCameraID: ::std::os::raw::c_int,
        direction: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "Start camera exposure. the following 4 API is usually used when long exposure required"]
    #[doc = "start exposure  and check the exposure status then get the data"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_BOOL bIsDark: means dark frame if there is mechanical shutter on the camera. otherwise useless"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_VIDEO_MODE_ACTIVE: video mode is working, you need to stop video capture first"]
    pub fn ASIStartExposure(
        iCameraID: ::std::os::raw::c_int,
        bIsDark: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "to cancel the long exposure which is on."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIStopExposure(iCameraID: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "to get the exposure status, work with ASIStartExposure."]
    #[doc = "you can read the data if get ASI_EXP_SUCCESS. or have to restart exposure again"]
    #[doc = "if get ASI_EXP_FAILED"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_EXPOSURE_STATUS *pExpStatus: the exposure status"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetExpStatus(
        iCameraID: ::std::os::raw::c_int,
        pExpStatus: *mut ASI_EXPOSURE_STATUS,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get data after exposure."]
    #[doc = "please make sure the buffer size is biger enough to hold one image"]
    #[doc = "otherwise the this API will crash"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "unsigned char* pBuffer, caller need to malloc the buffer, make sure the size is big enough"]
    #[doc = "the size in byte:"]
    #[doc = "8bit mono:width*height"]
    #[doc = "16bit mono:width*height*2"]
    #[doc = "RGB24:width*height*3"]
    #[doc = ""]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_TIMEOUT: no image get and timeout"]
    pub fn ASIGetDataAfterExp(
        iCameraID: ::std::os::raw::c_int,
        pBuffer: *mut ::std::os::raw::c_uchar,
        lBuffSize: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get camera id stored in flash, only available for USB3.0 camera"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_ID* pID: pointer to ID"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetID(iCameraID: ::std::os::raw::c_int, pID: *mut ASI_ID) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "write camera id to flash, only available for USB3.0 camera"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_ID ID: ID"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASISetID(iCameraID: ::std::os::raw::c_int, ID: ASI_ID) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get pre-setting parameter"]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "Offset_HighestDR: offset at highest dynamic range,"]
    #[doc = "Offset_UnityGain: offset at unity gain"]
    #[doc = "int *Gain_LowestRN, *Offset_LowestRN: gain and offset at lowest read noise"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetGainOffset(
        iCameraID: ::std::os::raw::c_int,
        pOffset_HighestDR: *mut ::std::os::raw::c_int,
        pOffset_UnityGain: *mut ::std::os::raw::c_int,
        pGain_LowestRN: *mut ::std::os::raw::c_int,
        pOffset_LowestRN: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get the frequently-used gain and offset"]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "pLGain: Low gain"]
    #[doc = "pMGain: Middle Gain"]
    #[doc = "pHGain: High Gain, the gain at the lowest read noise"]
    #[doc = "pHOffset: Offset at the lowest read noise"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetLMHGainOffset(
        iCameraID: ::std::os::raw::c_int,
        pLGain: *mut ::std::os::raw::c_int,
        pMGain: *mut ::std::os::raw::c_int,
        pHGain: *mut ::std::os::raw::c_int,
        pHOffset: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Descriptions:"]
    #[doc = "get version string, like \"1, 13, 0503\""]
    pub fn ASIGetSDKVersion() -> *mut ::std::os::raw::c_char;

    #[doc = "Description:"]
    #[doc = "Get the camera supported mode, only need to call when the IsTriggerCam in the CameraInfo is true."]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_SUPPORTED_MODE: the camera supported mode"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetCameraSupportMode(
        iCameraID: ::std::os::raw::c_int,
        pSupportedMode: *mut ASI_SUPPORTED_MODE,
    ) -> ::std::os::raw::c_int;

    #[doc = "Description:"]
    #[doc = "Get the camera current mode, only need to call when the IsTriggerCam in the CameraInfo is true"]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_CAMERA_MODE *mode: the current camera mode"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    pub fn ASIGetCameraMode(
        iCameraID: ::std::os::raw::c_int,
        mode: *mut ASI_CAMERA_MODE,
    ) -> ::std::os::raw::c_int;

    #[doc = "Description:"]
    #[doc = "Set the camera mode, only need to call when the IsTriggerCam in the CameraInfo is true"]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_CAMERA_MODE: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_SEQUENCE : camera is in capture now, need to stop capture first."]
    #[doc = "ASI_ERROR_INVALID_MODE  : mode is out of boundary or this camera do not support this mode"]
    pub fn ASISetCameraMode(
        iCameraID: ::std::os::raw::c_int,
        mode: ASI_CAMERA_MODE,
    ) -> ::std::os::raw::c_int;

    #[doc = "Description:"]
    #[doc = "Send out a softTrigger. For edge trigger, it only need to set true which means send a"]
    #[doc = "rising trigger to start exposure. For level trigger, it need to set true first means"]
    #[doc = "start exposure, and set false means stop exposure.it only need to call when the"]
    #[doc = "IsTriggerCam in the CameraInfo is true"]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_BOOL starts:send a softTrigger start/stop signal"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    pub fn ASISendSoftTrigger(
        iCameraID: ::std::os::raw::c_int,
        bStart: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    #[doc = "Description:"]
    #[doc = "Get a serial number from a camera."]
    #[doc = "It is 8 ASCII characters, you need to print it in hexadecimal."]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty"]
    #[doc = "ASI_SN* pSN: pointer to SN"]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_GENERAL_ERROR : camera does not have Serial Number"]
    pub fn ASIGetSerialNumber(
        iCameraID: ::std::os::raw::c_int,
        pSN: *mut ASI_SN,
    ) -> ::std::os::raw::c_int;

    #[doc = "Description:"]
    #[doc = "Config the output pin (A or B) of Trigger port. If lDuration <= 0, this output pin will be closed."]
    #[doc = "Only need to call when the IsTriggerCam in the CameraInfo is true"]
    #[doc = ""]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty."]
    #[doc = "ASI_TRIG_OUTPUT_STATUS pin: Select the pin for output"]
    #[doc = "ASI_BOOL bPinHigh: If true, the selected pin will output a high level as a signal"]
    #[doc = "when it is effective. Or it will output a low level as a signal."]
    #[doc = "long lDelay: the time between the camera receive a trigger signal and the output"]
    #[doc = "of the valid level.From 0 microsecond to 2000*1000*1000 microsecond."]
    #[doc = "long lDuration: the duration time of the valid level output.From 0 microsecond to"]
    #[doc = "2000*1000*1000 microsecond."]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_GENERAL_ERROR : the parameter is not right"]
    pub fn ASISetTriggerOutputIOConf(
        iCameraID: ::std::os::raw::c_int,
        pin: ASI_TRIG_OUTPUT_PIN,
        bPinHigh: ::std::os::raw::c_int,
        lDelay: ::std::os::raw::c_long,
        lDuration: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;

    #[doc = "Description:"]
    #[doc = "Get the output pin configuration, only need to call when the IsTriggerCam in the CameraInfo is true"]
    #[doc = "Paras:"]
    #[doc = "int CameraID: this is get from the camera property use the API ASIGetCameraProperty."]
    #[doc = "ASI_TRIG_OUTPUT_STATUS pin: Select the pin for getting the configuration"]
    #[doc = "ASI_BOOL *bPinAHigh: Get the current status of valid level."]
    #[doc = "long *lDelay: get the time between the camera receive a trigger signal and the output of the valid level."]
    #[doc = "long *lDuration: get the duration time of the valid level output."]
    #[doc = ""]
    #[doc = "return:"]
    #[doc = "ASI_SUCCESS : Operation is successful"]
    #[doc = "ASI_ERROR_CAMERA_CLOSED : camera didn't open"]
    #[doc = "ASI_ERROR_INVALID_ID  :no camera of this ID is connected or ID value is out of boundary"]
    #[doc = "ASI_ERROR_GENERAL_ERROR : the parameter is not right"]
    pub fn ASIGetTriggerOutputIOConf(
        iCameraID: ::std::os::raw::c_int,
        pin: ASI_TRIG_OUTPUT_PIN,
        bPinHigh: *mut ::std::os::raw::c_int,
        lDelay: *mut ::std::os::raw::c_long,
        lDuration: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}