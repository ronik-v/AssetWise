mod trade_report;
mod core;
mod ui;

use crate::core::signals::signal::Signal;
use crate::ui::app::AssetWise;


fn main() {
    let app = AssetWise::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
