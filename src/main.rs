#![feature(cstr_from_bytes_until_nul)]
use std::io::{stdin, stdout, Write};

use asi_camera2::ASIGetCameraProperty;

use crate::asi_camera2::*;

mod asi_camera2;

const BAYER_TYPES: [&str; 4] = ["RG", "BG", "GR", "GB"];

fn main() {
    unsafe { asi_setup() };
}

unsafe fn asi_setup() {
    let connected_cameras = asi_camera2::ASIGetNumOfConnectedCameras();
    if connected_cameras == 0 {
        println!("no camera connected!");
        return;
    } else {
        println!("found {} cameras", connected_cameras);
    }

    let mut cam_info = asi_camera2::ASI_CAMERA_INFO::default();

    println!("attached cameras: {{");
    for i in 0..connected_cameras {
        ASIGetCameraProperty(&mut cam_info, i);
        println!("{} : {},", i, cam_info.name().unwrap());
    }
    println!("}}");

    print!("select one camera: ");
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).unwrap();
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    let cam_id: i32 = s.parse().unwrap_or_else(|_| panic!("{} is not a number.", s));
    if cam_id > connected_cameras {
        panic!("no camera id is {}", cam_id);
    }

    if ASIOpenCamera(cam_id) != ASI_ERROR_CODE_ASI_SUCCESS {
        panic!("Error occured during opening camera {}", cam_id);
    }

    if ASIInitCamera(cam_id) != ASI_ERROR_CODE_ASI_SUCCESS {
        panic!("Error occured during initializing camera {}", cam_id);
    }

    ASIGetCameraPropertyByID(cam_id, &mut cam_info);
    println!("{} information", cam_info.name().unwrap());
    println!("resolution: {}x{}", cam_info.MaxWidth, cam_info.MaxHeight);

    if cam_info.IsColorCam == ASI_BOOL_ASI_TRUE {
        println!(
            "camera type: Color (bayer pattern {})",
            BAYER_TYPES[cam_info.BayerPattern as usize]
        )
    } else {
        println!("camera type: Mono")
    }

    println!("camera controls: [");
    let mut ctrl_caps = ASI_CONTROL_CAPS::default();
    let mut num_of_ctrls = 0;
    ASIGetNumOfControls(cam_id, &mut num_of_ctrls);
    for i in 0..num_of_ctrls {
        ASIGetControlCaps(cam_id, i, &mut ctrl_caps);
        println!("    {},", ctrl_caps.name().unwrap());
    }
    println!("]");

    let mut ltemp = 0;
    let mut b_auto = ASI_BOOL_ASI_FALSE;
    ASIGetControlValue(
        cam_id,
        ASI_CONTROL_TYPE_ASI_TEMPERATURE,
        &mut ltemp,
        &mut b_auto,
    );
    println!("camera temperature: {}", ltemp as f64 / 10.0);

    ASICloseCamera(cam_id);
}
