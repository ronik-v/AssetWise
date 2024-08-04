#![allow(warnings)]
mod trade_report;
mod core;

use eframe::{egui, epi};
use egui::plot::{Line, Plot, Value, Values};

#[derive(PartialEq, Default)]
enum Page {
    #[default]
    Home,
    Strategy,
    Settings,
}

#[derive(Default)]
struct MyApp {
    current_page: Page,
    ticker: String,
    strategy: String,
    theme: egui::Visuals,
    chart_type: ChartType,
}

#[derive(PartialEq)]
enum ChartType {
    Line,
    Candlestick,
}

impl Default for ChartType {
    fn default() -> Self {
        ChartType::Line
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, _: &epi::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Quantum Trade Lab");
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    if ui.button("⚙").clicked() {
                        self.current_page = Page::Settings;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Home => self.show_home(ui),
                Page::Strategy => self.show_strategy(ui),
                Page::Settings => self.show_settings(ui, ctx),
            }
        });
    }

    fn name(&self) -> &str {
        "Quantum Trade Lab"
    }
}

impl MyApp {
    fn show_home(&mut self, ui: &mut egui::Ui) {
        ui.heading("Добро пожаловать в торгового робота!");
        if ui.button("Перейти к анализу стратегий").clicked() {
            self.current_page = Page::Strategy;
        }
    }

    fn show_strategy(&mut self, ui: &mut egui::Ui) {
        ui.heading("Анализ стратегий");
        ui.horizontal(|ui| {
            ui.label("Название тикера:");
            ui.text_edit_singleline(&mut self.ticker);
        });
        ui.horizontal(|ui| {
            ui.label("Название стратегии:");
            egui::ComboBox::from_label("Выберите стратегию")
                .selected_text(&self.strategy)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.strategy, "Скользящие средние".to_string(), "Скользящие средние");
                    ui.selectable_value(&mut self.strategy, "Авторегрессионная скользящая средняя".to_string(), "Авторегрессионная скользящая средняя");
                    ui.selectable_value(&mut self.strategy, "Система направленного движения".to_string(), "Система направленного движения");
                });
        });

        ui.separator();

        // Пример графика
        let data: Vec<Value> = (0..100)
            .map(|x| Value::new(x as f64, (x as f64).sin()))
            .collect();

        Plot::new("Chart")
            .view_aspect(2.0)
            .show(ui, |plot_ui| {
                match self.chart_type {
                    ChartType::Line => {
                        plot_ui.line(Line::new(Values::from_values(data)).color(egui::Color32::from_rgb(200, 100, 100)));
                    }
                    ChartType::Candlestick => {
                        // Add code for using Japanese candles
                    }
                }
            });

        if ui.button("Назад").clicked() {
            self.current_page = Page::Home;
        }
    }

    fn show_settings(&mut self, ui: &mut egui::Ui, ctx: &egui::CtxRef) {
        ui.heading("Настройки");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Тема:");
            if ui.button("Светлая").clicked() {
                self.theme = egui::Visuals::light();
                ctx.set_visuals(self.theme.clone());
            }
            if ui.button("Тёмная").clicked() {
                self.theme = egui::Visuals::dark();
                ctx.set_visuals(self.theme.clone());
            }
        });

        ui.horizontal(|ui| {
            ui.label("Формат графика:");
            if ui.button("Обычный график").clicked() {
                self.chart_type = ChartType::Line;
            }
            if ui.button("Японские свечи").clicked() {
                self.chart_type = ChartType::Candlestick;
            }
        });

        if ui.button("Назад на главную").clicked() {
            self.current_page = Page::Home;
        }

        if ui.button("Назад к стратегиям").clicked() {
            self.current_page = Page::Strategy;
        }
    }
}

fn main() {
    let app = MyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
