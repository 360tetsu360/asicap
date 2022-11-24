#![feature(cstr_from_bytes_until_nul)]
use std::io::{stdin, stdout, Write};

use crate::asi::asi_api::*;

mod asi;

const BAYER_TYPES: [&str; 4] = ["RG", "BG", "GR", "GB"];

fn main() {
    unsafe { asi_setup() };
}

unsafe fn asi_setup() {
    let connected_cameras = get_num_of_connected_cameras();
    if connected_cameras == 0 {
        println!("no camera connected!");
        return;
    } else {
        println!("found {} cameras", connected_cameras);
    }

    println!("attached cameras: {{");
    for i in 0..connected_cameras {
        let cam_info = get_camera_property(i).unwrap();
        println!("{} : {},", i, cam_info.name);
    }
    println!("}}");

    print!("select one camera: ");
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).unwrap();
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    let cam_index: i32 = s
        .parse()
        .unwrap_or_else(|_| panic!("{} is not a number.", s));
    if cam_index > connected_cameras {
        panic!("no camera index is {}", cam_index);
    }

    let cam_info = get_camera_property(cam_index).unwrap();
    let cam_id = cam_info.camera_id;

    open_camera(cam_id).unwrap();
    init_camera(cam_id).unwrap();

    println!("{} information", cam_info.name);
    println!("resolution: {}x{}", cam_info.max_width, cam_info.max_height);

    if cam_info.is_color_cam {
        println!(
            "camera type: Color (bayer pattern {})",
            BAYER_TYPES[cam_info.bayer_pattern as usize]
        )
    } else {
        println!("camera type: Mono")
    }

    println!("camera controls: [");
    let num_of_ctrls = get_num_of_controls(cam_id).unwrap();
    for i in 0..num_of_ctrls {
        let ctrl_caps = get_control_caps(cam_id, i).unwrap();
        println!("    {},", ctrl_caps.name);
    }
    println!("]");

    let (ltemp, _) = get_control_value(cam_id, ASIControlType::Temperature).unwrap();
    println!("camera temperature: {}", ltemp as f64 / 10.0);

    close_camera(cam_id).unwrap();
}
