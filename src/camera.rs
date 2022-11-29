#![allow(dead_code)]

use std::{collections::HashMap, error::Error};

use crate::asi::asi_api::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub info: ASICameraInfo,
}

impl Camera {
    pub fn new(camera_index: i32) -> Result<(Self, i32), Box<dyn Error>> {
        let info = get_camera_property(camera_index)?;
        let id = info.camera_id;
        open_camera(id)?;
        init_camera(id)?;
        Ok((Self { info }, id))
    }

    pub fn get_control_caps(&self) -> Result<Vec<ASIControlCaps>, Box<dyn Error>> {
        todo!();
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
            let (camera, id) = Camera::new(cam_index)?;
            cam_hash.insert(id, camera);
        }

        Ok(Self { cams: cam_hash })
    }

    pub fn connected_cams(&self) -> usize {
        self.cams.len()
    }

    pub fn get_control_caps(&self, id: i32) -> Result<Vec<ASIControlCaps>, Box<dyn Error>> {
        self.cams.get(&id).unwrap().get_control_caps()
    }
}
