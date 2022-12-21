use eframe::egui;

pub fn show_dialog(
    ctx: &egui::Context,
    title: &str,
    close_button: bool,
    close: &mut bool,
    text: &str,
) -> bool {
    let anchor = egui::Align2::CENTER_CENTER;

    let mut window = egui::Window::new(title)
        .id(egui::Id::new("dialog")) // required since we change the title
        .resizable(false)
        .collapsible(false)
        .title_bar(true)
        .enabled(true)
        .anchor(anchor, [0., 0.]);

    if close_button {
        window = window.open(close);
    }

    let mut clicked = false;

    window.show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.label(text);
            clicked = ui.button("Ok").clicked();
        });
    });
    clicked
}
