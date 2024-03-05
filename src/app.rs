use egui::{Button, Layout, RichText, Vec2};
use log::error;

const BUTTON_SIZE: Vec2 = Vec2::new(60.0, 40.0);

// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Default)]
pub struct CalculatorApp {
    /// The number currently being entered
    value: Option<f64>,
    /// The current possibly partial solution
    answer: Option<f64>,
    /// The last operator key pressed
    last_operation: Option<Operator>,
    /// The current error if any
    error_message: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}
impl Operator {
    fn binary_operation(&self, op1: f64, op2: f64) -> f64 {
        match self {
            Operator::Add => op1 + op2,
            Operator::Subtract => op1 - op2,
            Operator::Multiply => op1 * op2,
            Operator::Divide => op1 / op2,
            Operator::Equal => todo!(),
        }
    }
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
            (None, None, _) => {
                self.answer = Some(0.);
                self.last_operation = Some(operator)
            }
            (None, Some(_), None) => {
                self.last_operation = Some(operator);
                self.answer = self.value;
                self.value = None;
            }
            (None, Some(_), Some(_)) => {
                error!("this should never happen because once we set a last_operation we should also set answer");
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
            (Some(op1), Some(op2), Some(op)) => match operator {
                OP::Add => todo!(),
                OP::Subtract => todo!(),
                OP::Multiply => todo!(),
                OP::Divide => todo!(),
                OP::Equal => {
                    // TODO 3: Right now all our operators are binary but we need to consider how we will deal with unary operators
                    self.answer = Some(op.binary_operation(op1, op2));
                    self.value = None;
                    self.last_operation = Some(operator);
                }
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

    /// Answer if it is Some followed by last_operator if it is Some unless last operator is "Equal"
    fn partial_result_display(&self) -> String {
        if Some(Operator::Equal) == self.last_operation {
            "".to_string()
        } else {
            format!(
                "{} {}",
                if let Some(answer) = self.answer {
                    answer.to_string()
                } else {
                    String::new()
                },
                if let Some(operator) = self.last_operation {
                    operator.to_string()
                } else {
                    String::new()
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
            // Small display top left
            ui.label(self.partial_result_display());

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
    #[case("add",
    vec![
        BT::Number(5.),
        BT::Operator(OP::Add),
        BT::Number(6.),
        BT::Operator(OP::Equal),
        BT::Operator(OP::Add),
        BT::Number(5.),
        BT::Operator(OP::Equal),
    ])]
    // TODO: Add test cases that start with each of the operators
    fn button_presses(#[case] name: &str, #[case] buttons: Vec<BT>) {
        let mut calc: CalculatorApp = Default::default();
        dbg!(name);
        // TODO 1: Continue working through the test cases
        for (i, button) in buttons.into_iter().enumerate() {
            calc.click_button(button);
            let name = format!("{name}-{i:02}");
            insta::assert_debug_snapshot!(name, calc);
        }
    }
}
