mod app;
mod cardioid;

use app::CardioidApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Cardioïde animée",
        options,
        Box::new(|_cc| Ok(Box::new(CardioidApp::default()))),
    )
}
