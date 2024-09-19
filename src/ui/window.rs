use std::sync::Arc;
use egui::Align2;
use egui::plot::{Bar, BarChart, Line, Plot, Polygon, Value, Values};
use crate::core::models::arima::Arima;
use crate::core::models::ktotm::KTOTM;
use crate::core::models::sma::Sma;
use crate::core::models::utility_function::UtilityFunction;
use crate::core::signals::signal::{Signal, TradeSignal};
use crate::core::utils::states::{States, Utility};
use crate::ui::app::AssetWise;
use crate::ui::enums::{ChartType, Page, STRATEGY_ARIMA, STRATEGY_KALMAN_FILTER, STRATEGY_SMA};
use crate::ui::load::{get_ticker_by_company_name, get_ticker_data};
use crate::ui::utils::is_valid_date;

impl AssetWise {
    fn show_error_window(&mut self, ctx: &egui::CtxRef) {
        let mut close_window = false;

        egui::Window::new("Ошибка")
            .open(&mut self.show_error_window)
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(&self.error_message);
                if ui.button("Закрыть").clicked() {
                    close_window = true;
                }
            });

        if close_window {
            self.show_error_window = false;
        }
    }

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
            ui.label("Название комппании:");
            ui.text_edit_singleline(&mut self.company_name);
        });
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("От (yyyy-mm--dd):");
            ui.text_edit_singleline(&mut self.date_start);
            ui.label("До (yyyy-mm--dd):");
            ui.text_edit_singleline(&mut self.date_end);
        });

        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("Название стратегии:");
            egui::ComboBox::from_label("")
                .selected_text(&self.strategy)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.strategy, STRATEGY_SMA.to_string(), STRATEGY_SMA);
                    ui.selectable_value(&mut self.strategy, STRATEGY_ARIMA.to_string(), STRATEGY_ARIMA);
                    ui.selectable_value(&mut self.strategy, STRATEGY_KALMAN_FILTER.to_string(), STRATEGY_KALMAN_FILTER);
                });
        });

        ui.add_space(20.0);

        if ui.button("Анализировать").clicked() {
            if !is_valid_date(&self.date_start) || !is_valid_date(&self.date_end) {
                self.show_error_window = true;
                self.error_message = "Неправильный формат даты. Используйте формат ГГГГ-ММ-ДД".to_string();

            } else {
                match get_ticker_by_company_name(&self.company_name) {
                    Ok(ticker) => {
                        println!("Ticker: {ticker}");
                        match get_ticker_data(Arc::new(ticker), self.date_start.clone(), self.date_end.clone(), 24) {
                            Ok(data) => {
                                self.ticker_data = Some(data);
                                self.signal = self.analyze_strategy();
                            }

                            Err(e) => {
                                self.show_error_window = true;
                                self.error_message = format!("Ошибка при загрузке данных: {}", e);
                            }
                        }
                    }
                    Err(err) => {
                        self.show_error_window = true;
                        self.error_message = format!("Ошибка: {}", err);
                    }
                }
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

                            if self.strategy == STRATEGY_SMA {
                                let sma5 = Sma::new(data.close.clone(), 5)
                                    .values().iter().enumerate().map(|(i, &p)| Value::new((i + 5) as f64, p)).collect();
                                let sma12 = Sma::new(data.close.clone(), 12)
                                    .values().iter().enumerate().map(|(i, &p)| Value::new((i + 12) as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(sma5)).color(egui::Color32::from_rgb(100, 200, 100)).name("SMA 5"));
                                plot_ui.line(Line::new(Values::from_values(sma12)).color(egui::Color32::from_rgb(100, 100, 200)).name("SMA 12"));
                            }

                            if self.strategy == STRATEGY_ARIMA {
                                let arima = Arima::new(data.close.clone());
                                let arima_values: Vec<Value> = arima.model_prediction_time_series().iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(arima_values)).color(egui::Color32::from_rgb(200, 150, 50)).name("ARIMA"));
                            }

                            if self.strategy == STRATEGY_KALMAN_FILTER {
                                let ktotm = KTOTM::new(data.close.clone());
                                let ktotm_values: Vec<Value> = ktotm.prediction_trend().iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();
                                plot_ui.line(Line::new(Values::from_values(ktotm_values)).color(egui::Color32::from_rgb(50, 150, 200)).name("KTOTM"));
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

                            if self.strategy == STRATEGY_SMA {
                                let sma5 = Sma::new(data.close.clone(), 5)
                                    .values().iter().enumerate().map(|(i, &p)| Value::new((i + 5) as f64, p)).collect();
                                let sma12 = Sma::new(data.close.clone(), 12)
                                    .values().iter().enumerate().map(|(i, &p)| Value::new((i + 12) as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(sma5)).color(egui::Color32::from_rgb(100, 200, 100)).name("SMA 5"));
                                plot_ui.line(Line::new(Values::from_values(sma12)).color(egui::Color32::from_rgb(100, 100, 200)).name("SMA 12"));
                            }

                            if self.strategy == STRATEGY_ARIMA {
                                let arima = Arima::new(data.close.clone());
                                let arima_values: Vec<Value> = arima.model_prediction_time_series().iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();

                                plot_ui.line(Line::new(Values::from_values(arima_values)).color(egui::Color32::from_rgb(200, 150, 50)).name("ARIMA"));
                            }

                            if self.strategy == STRATEGY_KALMAN_FILTER {
                                let ktotm = KTOTM::new(data.close.clone());
                                let ktotm_values: Vec<Value> = ktotm.prediction_trend().iter().enumerate().map(|(i, &p)| Value::new(i as f64, p)).collect();
                                plot_ui.line(Line::new(Values::from_values(ktotm_values)).color(egui::Color32::from_rgb(50, 150, 200)).name("KTOTM"));
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

        self.show_error_window(ctx);
    }

    pub(crate) fn show_settings(&mut self, ui: &mut egui::Ui, ctx: &egui::CtxRef) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Настройки приложения");

            if ui.button("Вернуться на главную").clicked() {
                self.current_page = Page::Home
            }

            ui.add_space(20.0);

            if ui.button("Вернуться к анализу").clicked() {
                self.current_page = Page::Strategy
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
                STRATEGY_SMA => {
                    let sma_5 = Sma::new(ticker_data.close.clone(), 5).values();
                    let sma_12 = Sma::new(ticker_data.close.clone(), 12).values();
                    Some((trade_signal.sma(sma_5, sma_12), UtilityFunction::new(ticker_data.clone(), 1.0).result()))
                }
                STRATEGY_ARIMA => {
                    let arima = Arima::new(ticker_data.close.clone());
                    Some((trade_signal.arima_or_kalman(arima.model_prediction_time_series()), UtilityFunction::new(ticker_data.clone(), 1.0).result()))
                }
                STRATEGY_KALMAN_FILTER => {
                    let ktotm = KTOTM::new(ticker_data.close.clone());
                    Some((trade_signal.arima_or_kalman(ktotm.prediction_trend()), UtilityFunction::new(ticker_data.clone(), 1.0).result()))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}