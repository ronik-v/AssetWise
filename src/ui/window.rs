use std::sync::Arc;
use egui::plot::{Bar, BarChart, Line, Plot, Polygon, Value, Values};
use crate::core::models::adx::Adx;
use crate::core::models::arima::Arima;
use crate::core::models::sma::Sma;
use crate::core::models::utility_function::UtilityFunction;
use crate::core::signals::signal::{Signal, TradeSignal};
use crate::core::utils::states::{States, Utility};
use crate::ui::app::AssetWise;
use crate::ui::enums::{ChartType, Page};
use crate::ui::load::get_ticker_data;

impl AssetWise {
    pub(crate) fn show_home(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Добро пожаловать в AssetWise!");
            ui.add_space(10.0);
            if ui.button("Перейти к анализу стратегий").clicked() {
                self.current_page = Page::Strategy;
            }
        });
    }

    pub(crate) fn show_strategy(&mut self, ui: &mut egui::Ui, ctx: &egui::CtxRef) {
        ui.add_space(20.0);
        ui.heading("Анализ стратегий");

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Название тикера:");
            ui.text_edit_singleline(&mut self.ticker);
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Название стратегии:");
            egui::ComboBox::from_label("")
                .selected_text(&self.strategy)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.strategy, "Скользящие средние".to_string(), "Скользящие средние");
                    ui.selectable_value(&mut self.strategy, "Авторегрессионная скользящая средняя".to_string(), "Авторегрессионная скользящая средняя");
                    ui.selectable_value(&mut self.strategy, "Система направленного движения".to_string(), "Система направленного движения");
                });
        });

        ui.add_space(20.0);
        if ui.button("Анализировать").clicked() {
            if let Ok(data) = get_ticker_data(Arc::new(self.ticker.clone()), "2024-01-01".to_string(), "2024-12-31".to_string(), 24) {
                self.ticker_data = Some(data);
                self.signal = self.analyze_strategy();
            }
        }

        ui.add_space(20.0);
        if let Some((signal, utility)) = &self.signal {
            let signal_text = match signal {
                States::BUY => ("Trade Signal: BUY", egui::Color32::GREEN),
                States::SELL => ("Trade Signal: SELL", egui::Color32::RED),
                States::WAIT => ("Trade Signal: WAIT", egui::Color32::YELLOW),
            };
            ui.colored_label(signal_text.1, signal_text.0);

            let utility_text = match utility {
                Utility::HOLD => ("Utility: HOLD", egui::Color32::GREEN),
                Utility::EXPECT => ("Utility: EXPECT", egui::Color32::YELLOW),
                Utility::ESCAPE => ("Utility: ESCAPE", egui::Color32::RED),
            };
            ui.colored_label(utility_text.1, utility_text.0);
        }

        ui.add_space(10.0);
        ui.separator();

        let window_size = ui.available_size();

        if let Some(data) = &self.ticker_data {
            Plot::new("Chart")
                .view_aspect(2.0)
                .width(window_size.x)
                .height(window_size.y * 0.75)
                .show(ui, |plot_ui| {
                    match self.chart_type {
                        ChartType::Line => {
                            let close_prices: Vec<Value> = data.close.iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();
                            plot_ui.line(Line::new(Values::from_values(close_prices)).color(egui::Color32::from_rgb(200, 100, 100)).name("Закрытие"));

                            if self.strategy == "Скользящие средние" {
                                let sma5 = Sma::new(data.close.clone(), 5)
                                    .values().iter().enumerate().map(|(i, &p)| Value::new((i + 5) as f64, p)).collect();
                                let sma12 = Sma::new(data.close.clone(), 12)
                                    .values().iter().enumerate().map(|(i, &p)| Value::new((i + 12) as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(sma5)).color(egui::Color32::from_rgb(100, 200, 100)).name("SMA 5"));
                                plot_ui.line(Line::new(Values::from_values(sma12)).color(egui::Color32::from_rgb(100, 100, 200)).name("SMA 12"));
                            }

                            if self.strategy == "Авторегрессионная скользящая средняя" {
                                let arima = Arima::new(data.close.clone());
                                let arima_values: Vec<Value> = arima.model_prediction_time_series().iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(arima_values)).color(egui::Color32::from_rgb(200, 150, 50)).name("ARIMA"));
                            }

                            if self.strategy == "Система направленного движения" {
                                let adx = Adx::new(data.clone(), 14);
                                let adx_values: Vec<Value> = adx.adx().iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(adx_values)).color(egui::Color32::from_rgb(50, 150, 200)).name("ADX"));
                            }
                        }
                        ChartType::Candlestick => {
                            for (i, (((&o, &c), &h), &l)) in data.open.iter().zip(&data.close).zip(&data.high).zip(&data.low).enumerate() {
                                let color = if c >= o { egui::Color32::GREEN } else { egui::Color32::RED };
                                plot_ui.line(Line::new(Values::from_values(vec![
                                    Value::new(i as f64, l),
                                    Value::new(i as f64, h),
                                ])).color(color));

                                plot_ui.polygon(Polygon::new(Values::from_values(vec![
                                    Value::new(i as f64 - 0.25, o),
                                    Value::new(i as f64 + 0.25, o),
                                    Value::new(i as f64 + 0.25, c),
                                    Value::new(i as f64 - 0.25, c),
                                ])).color(color));
                            }

                            if self.strategy == "Скользящие средние" {
                                let sma5 = Sma::new(data.close.clone(), 5)
                                    .values().iter().enumerate().map(|(i, &p)| Value::new((i + 5) as f64, p)).collect();
                                let sma12 = Sma::new(data.close.clone(), 12)
                                    .values().iter().enumerate().map(|(i, &p)| Value::new((i + 12) as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(sma5)).color(egui::Color32::from_rgb(100, 200, 100)).name("SMA 5"));
                                plot_ui.line(Line::new(Values::from_values(sma12)).color(egui::Color32::from_rgb(100, 100, 200)).name("SMA 12"));
                            }

                            if self.strategy == "Авторегрессионная скользящая средняя" {
                                let arima = Arima::new(data.close.clone());
                                let arima_values: Vec<Value> = arima.model_prediction_time_series().iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(arima_values)).color(egui::Color32::from_rgb(200, 150, 50)).name("ARIMA"));
                            }

                            if self.strategy == "Система направленного движения" {
                                let adx = Adx::new(data.clone(), 14);
                                let adx_values: Vec<Value> = adx.adx().iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(adx_values)).color(egui::Color32::from_rgb(50, 150, 200)).name("ADX"));
                            }
                        }
                    }
                });

            ui.add_space(40.0);

            Plot::new("Closing Price Histogram")
                .view_aspect(4.0)
                .width(window_size.x)
                .height(window_size.y * 0.25)
                .show(ui, |plot_ui| {
                    let close_prices_histogram: Vec<Bar> = data.close.iter().enumerate().map(|(i, &p)| {
                        Bar::new(i as f64, p).width(0.5)
                    }).collect();

                    let chart = BarChart::new(close_prices_histogram).name("Закрытие цен");
                    plot_ui.bar_chart(chart);
                });
        }
    }

    pub(crate) fn show_settings(&mut self, ui: &mut egui::Ui, ctx: &egui::CtxRef) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Настройки приложения");

            if ui.button("Вернуться на главную").clicked() {
                self.current_page = Page::Home
            }

            ui.horizontal(|ui| {
                ui.label("Тема:");
                egui::ComboBox::from_id_source("theme_combo_box")
                    .selected_text(if self.theme == egui::Visuals::dark() { "Dark" } else { "Light" })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.theme, egui::Visuals::dark(), "Dark");
                        ui.selectable_value(&mut self.theme, egui::Visuals::light(), "Light");
                    });
            });

            if ui.button("Сохранить настройки").clicked() {
                ctx.set_visuals(self.theme.clone());
            }

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Тип графика:");
                egui::ComboBox::from_id_source("chart_type_combo_box")
                    .selected_text(if self.chart_type == ChartType::Line { "Линейный" } else { "Свечной" })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.chart_type, ChartType::Line, "Линейный");
                        ui.selectable_value(&mut self.chart_type, ChartType::Candlestick, "Свечной");
                    });
            });
        });
    }


    fn analyze_strategy(&self) -> Option<(States, Utility)> {
        if let Some(ticker_data) = &self.ticker_data {
            let trade_signal = TradeSignal;
            match self.strategy.as_str() {
                "Скользящие средние" => {
                    let sma_5 = Sma::new(ticker_data.close.clone(), 5).values();
                    let sma_12 = Sma::new(ticker_data.close.clone(), 12).values();
                    Some((trade_signal.sma(sma_5, sma_12), UtilityFunction::new(ticker_data.clone(), 1.0).result()))
                }
                "Авторегрессионная скользящая средняя" => {
                    let arima = Arima::new(ticker_data.close.clone());
                    Some((trade_signal.arima(arima.model_prediction_time_series()), UtilityFunction::new(ticker_data.clone(), 1.0).result()))
                }
                "Система направленного движения" => {
                    let adx = Adx::new(ticker_data.clone(), 14);
                    let di_plus = adx.directional_indicators(true);
                    let di_minus = adx.directional_indicators(false);
                    let adx_values = adx.adx();
                    Some((trade_signal.adx(di_plus, di_minus, adx_values, false), UtilityFunction::new(ticker_data.clone(), 1.0).result()))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}