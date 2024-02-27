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
enum SpecialButton {
    Clear,
}

enum ButtonType {
    Number(f64),
    Operator(Operator),
    Special(SpecialButton),
}

impl std::fmt::Display for SpecialButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialButton::Clear => write!(f, "C"),
        }
    }
}
impl std::fmt::Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonType::Number(value) => write!(f, "{value}"),
            ButtonType::Operator(value) => write!(f, "{value}"),
            ButtonType::Special(value) => write!(f, "{value}"),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "x",
            Operator::Divide => "/",
            Operator::Equal => "=",
        };
        write!(f, "{value}")
    }
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
            (None, None, _) => self.last_operation = Some(operator),
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

    fn create_button(&mut self, ui: &mut egui::Ui, button_value: ButtonType) {
        if ui
            .add(Button::new(format!("{button_value}")).min_size(BUTTON_SIZE))
            .clicked()
        {
            self.click_button(button_value);
        }
    }

    fn click_button(&mut self, button_value: ButtonType) {
        match button_value {
            ButtonType::Number(number) => self.click_number(number),
            ButtonType::Operator(operator) => self.click_operator(operator),
            ButtonType::Special(special) => self.click_special(special),
        }
    }

    fn click_special(&mut self, special: SpecialButton) {
        match special {
            SpecialButton::Clear => *self = CalculatorApp::default(),
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
            ui.label(if let Some(operator) = self.last_operation {
                format!("{operator}")
            } else {
                String::from("")
            });
            ui.allocate_ui_with_layout(
                Vec2::new(250., 40.),
                Layout::right_to_left(egui::Align::BOTTOM),
                |ui| ui.label(RichText::new(self.display_value()).heading()),
            );

            ui.horizontal(|ui| {
                self.create_button(ui, ButtonType::Number(7.));
                self.create_button(ui, ButtonType::Number(8.));
                self.create_button(ui, ButtonType::Number(9.));
                self.create_button(ui, ButtonType::Operator(Operator::Divide));
            });

            ui.horizontal(|ui| {
                self.create_button(ui, ButtonType::Number(4.));
                self.create_button(ui, ButtonType::Number(5.));
                self.create_button(ui, ButtonType::Number(6.));
                self.create_button(ui, ButtonType::Operator(Operator::Multiply));
            });

            ui.horizontal(|ui| {
                self.create_button(ui, ButtonType::Number(1.));
                self.create_button(ui, ButtonType::Number(2.));
                self.create_button(ui, ButtonType::Number(3.));
                self.create_button(ui, ButtonType::Operator(Operator::Subtract));
            });

            ui.horizontal(|ui| {
                self.create_button(ui, ButtonType::Number(0.));
                self.create_button(ui, ButtonType::Special(SpecialButton::Clear));
                self.create_button(ui, ButtonType::Operator(Operator::Equal));
                self.create_button(ui, ButtonType::Operator(Operator::Add));
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{ButtonType as BT, CalculatorApp, Operator as OP};

    #[test]
    fn empty_at_startup() {
        let calc: CalculatorApp = Default::default();
        insta::assert_debug_snapshot!(calc);
    }

    #[rstest]
    #[case::add(vec![
        BT::Number(5.),
        BT::Operator(OP::Add),
        BT::Number(6.),
        BT::Operator(OP::Equal),
        BT::Operator(OP::Add),
        BT::Number(5.),
        BT::Operator(OP::Equal),
    ])]
    fn add(#[case] buttons: Vec<BT>) {
        let mut calc: CalculatorApp = Default::default();
        for button in buttons {
            calc.click_button(button);
            insta::assert_debug_snapshot!(calc);
        }
    }
}
