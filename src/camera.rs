use std::{collections::HashMap, error::Error};

use crate::asi::asi_api::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub info: ASICameraInfo,
}

impl Camera {
    pub fn new(camera_index: i32) -> Result<Self, Box<dyn Error>> {
        let info = get_camera_property(camera_index)?;
        open_camera(info.camera_id)?;
        init_camera(info.camera_id)?;
        Ok(Self { info })
    }
}

impl Drop for Camera {
    fn drop(&mut self) {
        close_camera(self.info.camera_id).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct CameraManager {
    cams: HashMap<i32, Camera>,
}

impl CameraManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let connected_cams = get_num_of_connected_cameras();

        let mut cam_hash = HashMap::new();
        for cam_index in 0..connected_cams {
            cam_hash.insert(cam_index, Camera::new(cam_index)?);
        }

        Ok(Self { cams: cam_hash })
    }

    pub fn connected_cams(&self) -> usize {
        self.cams.len()
    }
}
