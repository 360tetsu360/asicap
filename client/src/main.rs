#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(dead_code)]

use std::net::SocketAddr;

use design::dialog::show_dialog;
use eframe::egui;
use futures::FutureExt;
use nets::NetworkManager;
use tab::*;

mod asi;
mod bytes;
mod design;
mod nets;
mod packets;
mod tab;

pub const MAGIC: &[u8] = b"A51C4P";

fn main() {
    // egui setup
    let options = eframe::NativeOptions::default();
    eframe::run_native("ASICap", options, Box::new(|cc| Box::new(MainApp::new(cc))));
}

enum MainAppState {
    Finding,
    Tabs,
}

struct FindingPageState {
    finding: bool,
    retry_msgbox_open: bool,
}

impl FindingPageState {
    fn new() -> Self {
        Self {
            finding: true,
            retry_msgbox_open: false,
        }
    }
}

struct MainApp {
    state: MainAppState,
    net_manager: NetworkManager,
    finging_page: FindingPageState,
    tab_manager: TabManager,
}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.2); //
        Self {
            state: MainAppState::Finding,
            net_manager: NetworkManager::new(),
            finging_page: FindingPageState::new(),
            tab_manager: TabManager::new(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.state {
            MainAppState::Finding => self.update_finding(ctx, frame),
            MainAppState::Tabs => self.update_main(ctx, frame),
        }
    }
}

impl MainApp {
    fn update_finding(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(finder) = self.net_manager.server_finder.as_mut() {
            match finder.now_or_never() {
                Some(Ok(status)) => {
                    println!("Connected!");
                    self.net_manager.server_finder = None;
                    self.finging_page.finding = false;

                    self.state = MainAppState::Tabs;
                    if self
                        .net_manager
                        .start_connection(SocketAddr::new(status.ip, status.port))
                        .is_err()
                    {
                        todo!()
                    }
                }
                Some(Err(_timeout)) => {
                    self.finging_page.retry_msgbox_open = true;
                    self.net_manager.server_finder = None;
                    self.finging_page.finding = false;
                }
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.finging_page.finding {
                ui.vertical_centered(|ui| {
                    ui.add(egui::Spinner::new());
                    ui.label(
                        egui::RichText::new("Finding Server...")
                            .font(egui::FontId::proportional(20.0)),
                    )
                });
            }
        });

        if self.finging_page.retry_msgbox_open {
            let clicked = show_dialog(ctx, "Error", false, None, "No server found");

            if clicked {
                self.net_manager.retry_find_server();
                self.finging_page.retry_msgbox_open = false;
                self.finging_page.finding = true;
            }
        }
    }

    fn update_main(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tab").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;

                if ui.button("+").clicked() {
                    let tab = Tab::new(self.tab_manager.tab_id_next, &mut self.net_manager);
                    self.tab_manager
                        .tabs
                        .insert(self.tab_manager.tab_id_next, tab);
                    self.tab_manager.tab_id_next += 1;
                }

                ui.separator();

                let mut selected = self.tab_manager.selected_tab_id;
                for (id, tab) in self.tab_manager.tabs.iter() {
                    if ui.selectable_label(*id == selected, &tab.title).clicked() {
                        selected = *id;
                    }
                    ui.separator();
                }
                self.tab_manager.selected_tab_id = selected;

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.tab_manager.selected_tab_id != -1 {
                self.tab_manager
                    .tabs
                    .get_mut(&self.tab_manager.selected_tab_id)
                    .unwrap()
                    .show(ctx, ui, &mut self.net_manager)
            }
        });
    }
}
