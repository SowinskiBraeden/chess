mod chess;
mod ui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_min_inner_size([480.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Chess",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ui::App::default()))
        }),
    )
}
