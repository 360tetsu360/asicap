use std::{collections::HashMap, pin::Pin};

use eframe::egui;
use futures::{channel::oneshot::Canceled, FutureExt};

use crate::{
    asi::camera::Camera,
    nets::NetworkManager,
    design::dialog::show_dialog,
    packets::{ConnectedCamerasPacket, Requests, Responses, OpenCameraStatusPacket},
};

type ServerResponse = Pin<Box<dyn futures::Future<Output = Result<Responses, Canceled>>>>;

pub struct TabManager {
    pub tab_id_next: i32,
    pub tabs: HashMap<i32, Tab>,
    pub selected_tab_id: i32,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tab_id_next: 0,
            tabs: HashMap::new(),
            selected_tab_id: -1,
        }
    }
}

pub enum OpenCameraFailed {
    NoCameraFound,
    CameraInUse,
}

impl From<OpenCameraStatusPacket> for OpenCameraFailed {
    fn from(packet: OpenCameraStatusPacket) -> Self {
        match packet {
            OpenCameraStatusPacket::Success => panic!(),
            OpenCameraStatusPacket::NoCameraFound => OpenCameraFailed::NoCameraFound,
            OpenCameraStatusPacket::CameraInUse => OpenCameraFailed::CameraInUse,
        }
    }
}

pub enum TabState {
    SelectCamera(Option<Vec<(Camera, bool)>>, Option<OpenCameraFailed>),
    ViewCamera,
}

pub struct Tab {
    pub id: i32,
    pub title: String,
    pub state: TabState,
    pub res_future_queue: Vec<ServerResponse>,
}

impl Tab {
    pub fn new(id: i32, netmanager: &mut NetworkManager) -> Self {
        Self {
            id,
            title: "New Tab".to_string(),
            state: TabState::SelectCamera(None, None),
            res_future_queue: vec![netmanager.request_api(Requests::GetConnectedCameras)],
        }
    }

    fn check_net_response(&mut self, _netmanager: &mut NetworkManager) {
        let mut received_responses = vec![];

        let mut not_received_responses = vec![];
        while let Some(mut res_future) = self.res_future_queue.pop() {
            if let Some(res) = (&mut res_future).now_or_never() {
                received_responses.push(res.unwrap());
            } else {
                not_received_responses.push(res_future);
            }
        }
        self.res_future_queue = not_received_responses;

        for response in received_responses {
            match response {
                Responses::ConnectedCameras(packet) => self.handle_connected_cam_res(packet),
                Responses::ControlValue(_) => todo!(),
                Responses::OpenCameraStatus(status) => {
                    if let OpenCameraStatusPacket::Success = status {

                    }else {
                        if let TabState::SelectCamera(_, failed) = &mut self.state {
                            *failed = Some(status.into());
                        }
                    }
                }
                Responses::ASIError(_) => todo!(),
                Responses::None => todo!(),
            }
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        netmanager: &mut NetworkManager,
    ) {
        self.check_net_response(netmanager);

        let mut connect_cam_id = None;
        match &mut self.state {
            TabState::SelectCamera(None, None) => {
                ui.vertical_centered(|ui| {
                    ui.add(egui::Spinner::new());
                    ui.label(
                        egui::RichText::new("Finding Camera...")
                            .font(egui::FontId::proportional(20.0)),
                    )
                });
            }
            TabState::SelectCamera(None, Some(_)) => panic!(),
            TabState::SelectCamera(Some(cams), failed) => {
                if cams.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("No camera found.")
                                .font(egui::FontId::proportional(20.0)),
                        );
                        if ui.button("reload").clicked() {
                            self.res_future_queue
                                .push(netmanager.request_api(Requests::GetConnectedCameras));
                        }
                    });
                } else {
                    egui::SidePanel::left("SelectCam")
                        .resizable(false)
                        .show(ctx, |ui| {
                            egui::Grid::new("Cameras")
                                .num_columns(3)
                                .spacing([8.0, 4.0])
                                .show(ui, |ui| {
                                    for (cam, show) in cams.iter_mut() {
                                        ui.label(&cam.info.name);
                                        if ui.button("Show").clicked() {
                                            *show = true;
                                        }
                                        ui.add_enabled_ui(!cam.connected, |ui| {
                                            if ui.button("Connect").clicked() {
                                                connect_cam_id = Some(cam.id);
                                            }
                                        });
                                        ui.end_row();
                                    }
                                });
                            ui.separator();
                            ui.vertical_centered(|ui| {
                                if ui.button("reload").clicked() {
                                    self.res_future_queue.push(
                                        netmanager.request_api(Requests::GetConnectedCameras),
                                    );
                                }
                            });
                        });
                }

                for (cam, show) in cams.iter_mut() {
                    if *show {
                        let window_title = if cam.connected {
                            format!("{} - Connected", &cam.info.name)
                        } else {
                            cam.info.name.clone()
                        };
                        egui::Window::new(window_title)
                            .scroll2([true, true])
                            .open(show)
                            .resizable(true)
                            .show(ctx, |ui| {
                                Self::info_grid(ui, cam);
                                ui.separator();
                                ui.vertical_centered(|ui| {
                                    ui.add_enabled_ui(!cam.connected, |ui| {
                                        if ui.button("Connect").clicked() {
                                            connect_cam_id = Some(cam.id);
                                        }
                                    });
                                });
                            });
                    }
                }

                if let Some(fail) = failed {
                    let err_msg = match fail {
                        OpenCameraFailed::NoCameraFound => "That camera was not available.",
                        OpenCameraFailed::CameraInUse => "That camera is in use.",
                    };

                    if show_dialog(ctx, "Error", false, None, err_msg) {
                        *failed = None;
                    }
                }
            }
            TabState::ViewCamera => todo!(),
        }

        if let Some(camera_id) = connect_cam_id {
            self.connect(camera_id, netmanager);
        }
    }

    fn handle_connected_cam_res(&mut self, mut packet: ConnectedCamerasPacket) {
        match &mut self.state {
            TabState::SelectCamera(cams,_) => {
                *cams = Some(vec![]);
                while let Some(cam) = packet.0.pop() {
                    cams.as_mut().unwrap().push((cam, false));
                }
            }
            TabState::ViewCamera => panic!(),
        }
    }

    fn info_grid(ui: &mut egui::Ui, cam: &Camera) {
        egui::Grid::new("Info")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("name");
                ui.label(&cam.info.name);
                ui.end_row();

                ui.label("id");
                ui.label(cam.info.camera_id.to_string());
                ui.end_row();

                ui.label("max height");
                ui.label(cam.info.max_height.to_string());
                ui.end_row();

                ui.label("max width");
                ui.label(cam.info.max_width.to_string());
                ui.end_row();

                ui.label("is color cam");
                ui.label(cam.info.is_color_cam.to_string());
                ui.end_row();

                ui.label("bayer pattern");
                ui.label(cam.info.bayer_pattern.to_string());
                ui.end_row();

                ui.label("supported binnings");
                ui.collapsing("Show binnings", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        egui::Grid::new("Binnings")
                            .num_columns(1)
                            .spacing([40.0, 4.0])
                            .show(ui, |ui| {
                                for bin in cam.info.supported_bins.iter() {
                                    ui.label(bin.to_string());
                                    ui.end_row();
                                }
                            })
                    });
                });
                ui.end_row();

                ui.label("supported video format");
                ui.collapsing("Show image types", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        egui::Grid::new("VideoFormats")
                            .num_columns(1)
                            .spacing([40.0, 4.0])
                            .show(ui, |ui| {
                                for imgtype in cam.info.supported_video_format.iter() {
                                    ui.label(imgtype.to_string());
                                    ui.end_row();
                                }
                            })
                    });
                });
                ui.end_row();

                ui.label("pixel size");
                ui.label(cam.info.pixel_size.to_string());
                ui.end_row();

                ui.label("mechanical shutter");
                ui.label(cam.info.mechanical_shutter.to_string());
                ui.end_row();

                ui.label("st4 port");
                ui.label(cam.info.st4_port.to_string());
                ui.end_row();

                ui.label("is cooler camera");
                ui.label(cam.info.is_cooler_cam.to_string());
                ui.end_row();

                ui.label("is usb3 host");
                ui.label(cam.info.is_usb3_host.to_string());
                ui.end_row();

                ui.label("is usb3 camera");
                ui.label(cam.info.is_usb3_camera.to_string());
                ui.end_row();

                ui.label("elec per adu");
                ui.label(cam.info.elec_per_adu.to_string());
                ui.end_row();

                ui.label("bit depth");
                ui.label(cam.info.bit_depth.to_string());
                ui.end_row();

                ui.label("is trigger cam");
                ui.label(cam.info.is_trigger_cam.to_string());
                ui.end_row();

                ui.label("control caps");
                ui.collapsing("Show control caps", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        egui::Grid::new("ControlCaps")
                            .num_columns(2)
                            .spacing([40.0, 4.0])
                            .show(ui, |ui| {
                                for control in cam.controls.iter() {
                                    ui.label(&control.name);
                                    ui.collapsing(format!("Show {} info", control.name), |ui| {
                                        ui.horizontal_wrapped(|ui| {
                                            egui::Grid::new(format!("{}Info", control.name))
                                                .num_columns(2)
                                                .spacing([40.0, 4.0])
                                                .striped(true)
                                                .show(ui, |ui| {
                                                    ui.label("name");
                                                    ui.label(&control.name);
                                                    ui.end_row();

                                                    ui.label("description");
                                                    ui.label(&control.description);
                                                    ui.end_row();

                                                    ui.label("max value");
                                                    ui.label(control.max_value.to_string());
                                                    ui.end_row();

                                                    ui.label("min value");
                                                    ui.label(control.min_value.to_string());
                                                    ui.end_row();

                                                    ui.label("default value");
                                                    ui.label(control.default_value.to_string());
                                                    ui.end_row();

                                                    ui.label("is auto supported");
                                                    ui.label(control.is_auto_supported.to_string());
                                                    ui.end_row();

                                                    ui.label("is writable");
                                                    ui.label(control.is_writable.to_string());
                                                    ui.end_row();

                                                    ui.label("control type");
                                                    ui.label(control.control_type.to_string());
                                                    ui.end_row();
                                                })
                                        });
                                    });
                                }
                            })
                    });
                });
            });
    }

    fn connect(&mut self, id: u32, netmanager: &mut NetworkManager) {
        self.res_future_queue
            .push(netmanager.request_api(Requests::OpenCamera(id)));
    }
}
