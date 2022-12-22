use eframe::egui;

use crate::{asi::camera::Camera, nets::NetworkManager};

pub struct ViewCameraApp {
    pub cam: Camera,
}

impl ViewCameraApp {
    pub fn new(cam: Camera) -> Self {
        Self { cam }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        netmanager: &mut NetworkManager,
    ) {
        egui::SidePanel::right("controls")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(&self.cam.info.name)
                            .font(egui::FontId::proportional(20.0)),
                    )
                });
                ui.separator();
            });
    }
}
