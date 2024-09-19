use eframe::epi;
use crate::core::data::moex_parser::Ticker;
use crate::core::utils::states::{States, Utility};
use crate::ui::enums::{ChartType, Page};

#[derive(Default)]
pub struct AssetWise {
    pub current_page: Page,
    pub company_name: String,
    pub strategy: String,
    pub theme: egui::Visuals,
    pub chart_type: ChartType,
    pub signal: Option<(States, Utility, Vec<States>)>,
    pub ticker_data: Option<Ticker>,
    pub date_start: String,
    pub date_end: String,
    pub show_error_window: bool,
    pub error_message: String,
}


impl Default for ChartType {
    fn default() -> Self {
        ChartType::Line
    }
}


impl epi::App for AssetWise {
    fn update(&mut self, ctx: &egui::CtxRef, _: &epi::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.heading("AssetWise");
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("⚙").clicked() {
                        self.current_page = Page::Settings;
                    }
                });
                ui.add_space(10.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Home => self.show_home(ui),
                Page::Strategy => self.show_strategy(ui, ctx),
                Page::Settings => self.show_settings(ui, ctx),
            }
        });
    }

    fn name(&self) -> &str {
        "AssetWise"
    }
}