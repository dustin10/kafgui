use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 900.0]),
        ..Default::default()
    };

    eframe::run_simple_native("KafGUI", options, move |ctx, _frame| {
        ctx.set_theme(egui::ThemePreference::Dark);

        egui::CentralPanel::default().show(ctx, |_ui| {});
    })
}
