use egui::{Button, Layout, RichText, Vec2};
use log::error;

const BUTTON_SIZE: Vec2 = Vec2::new(60.0, 40.0);

// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Default)]
pub struct CalculatorApp {
    value: Option<f64>,
    answer: Option<f64>,
    last_operation: Option<Operator>,
    error_message: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

impl CalculatorApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // TODO if we decide to show history then we can enable this but off for now
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }

    fn click_number(&mut self, number: f64) {
        self.value = if let Some(value) = self.value {
            Some(value * 10.0 + number)
        } else {
            Some(number)
        };
    }

    fn click_operator(&mut self, operator: Operator) {
        type OP = Operator;
        match (self.answer, self.value, self.last_operation) {
            (None, None, _) => (),
            (None, Some(_), None) => {
                self.last_operation = Some(operator);
                self.answer = self.value;
                self.value = None;
            }
            (None, Some(_), Some(_)) => {
                self.log_debug_info_for_operator_click(operator);
                self.error_message = Some("Err: Unreachable".to_owned());
            }
            (Some(_), None, None) => {
                self.log_debug_info_for_operator_click(operator);
                self.error_message = Some("Err: Unreachable".to_owned());
            }
            (Some(_), None, Some(_)) => self.last_operation = Some(operator),
            (Some(_), Some(_), None) => {
                self.log_debug_info_for_operator_click(operator);
                self.error_message = Some("Err: Unreachable".to_owned());
            }
            (Some(_), Some(_), Some(_)) => match operator {
                OP::Add => todo!(),
                OP::Subtract => todo!(),
                OP::Multiply => todo!(),
                OP::Divide => todo!(),
                OP::Equal => todo!(),
            },
        }
    }

    fn log_debug_info_for_operator_click(&mut self, operator: Operator) {
        error!("[current operator received: {:?}] {:?}", operator, self);
    }

    fn display_value(&self) -> String {
        if let Some(error_message) = &self.error_message {
            error_message.clone()
        } else {
            format!(
                "{}",
                if let Some(value) = self.value {
                    value
                } else {
                    self.answer.unwrap_or_default()
                }
            )
        }
    }
}

impl eframe::App for CalculatorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.allocate_ui_with_layout(
                Vec2::new(250., 40.),
                Layout::right_to_left(egui::Align::BOTTOM),
                |ui| ui.label(RichText::new(self.display_value()).heading()),
            );

            ui.horizontal(|ui| {
                // TODO 1: Change buttons to be created by a function
                if ui.add(Button::new("7").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(7.0);
                };
                if ui.add(Button::new("8").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(8.0);
                };
                if ui.add(Button::new("9").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(9.0);
                };
                if ui.add(Button::new("/").min_size(BUTTON_SIZE)).clicked() {};
            });

            ui.horizontal(|ui| {
                if ui.add(Button::new("4").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(4.0);
                };
                if ui.add(Button::new("5").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(5.0);
                };
                if ui.add(Button::new("6").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(6.0);
                };
                if ui.add(Button::new("x").min_size(BUTTON_SIZE)).clicked() {}
            });

            ui.horizontal(|ui| {
                if ui.add(Button::new("1").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(1.0);
                };
                if ui.add(Button::new("2").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(2.0);
                };
                if ui.add(Button::new("3").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(3.0);
                };
                if ui.add(Button::new("-").min_size(BUTTON_SIZE)).clicked() {};
            });

            ui.horizontal(|ui| {
                if ui.add(Button::new("0").min_size(BUTTON_SIZE)).clicked() {
                    self.click_number(0.0);
                };
                if ui.add(Button::new("C").min_size(BUTTON_SIZE)).clicked() {
                    self.answer = None;
                    self.value = None;
                    // TODO update C for new fields
                    // TODO ensure we don't miss fields in the future
                };
                if ui.add(Button::new("=").min_size(BUTTON_SIZE)).clicked() {};
                if ui.add(Button::new("+").min_size(BUTTON_SIZE)).clicked() {
                    self.click_operator(Operator::Add);
                };
            });
        });
    }
}

// TODO add tests to ensure expected operation and record how program is expected to work
