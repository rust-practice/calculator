// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
pub struct CalculatorApp {
    value: Option<f64>,
    answer: Option<f64>,
    last_operation: Option<Operator>,
    error_message: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
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
        match operator {
            Operator::Add => todo!(),
            Operator::Subtract => todo!(),
            Operator::Multiply => todo!(),
            Operator::Divide => todo!(),
            Operator::Equal => todo!(),
        }
    }
}

impl eframe::App for CalculatorApp {
    /// Called by the frame work to save state before shutdown.
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
            ui.heading("Calculator");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                if let Some(error_message) = &self.error_message {
                    ui.label(error_message)
                } else {
                    ui.label(&format!(
                        "{}",
                        if let Some(value) = self.value {
                            value
                        } else {
                            self.answer.unwrap_or_default()
                        }
                    ))
                };
            });

            ui.horizontal(|ui| {
                if ui.button("7").clicked() {
                    self.click_number(7.0);
                };
                if ui.button("8").clicked() {
                    self.click_number(8.0);
                };
                if ui.button("9").clicked() {
                    self.click_number(9.0);
                };
                if ui.button("/").clicked() {};
            });

            ui.horizontal(|ui| {
                if ui.button("4").clicked() {
                    self.click_number(4.0);
                };
                if ui.button("5").clicked() {
                    self.click_number(5.0);
                };
                if ui.button("6").clicked() {
                    self.click_number(6.0);
                };
                if ui.button("x").clicked() {};
            });

            ui.horizontal(|ui| {
                if ui.button("1").clicked() {
                    self.click_number(1.0);
                };
                if ui.button("2").clicked() {
                    self.click_number(2.0);
                };
                if ui.button("3").clicked() {
                    self.click_number(3.0);
                };
                if ui.button("-").clicked() {};
            });

            ui.horizontal(|ui| {
                if ui.button("0").clicked() {
                    self.click_number(0.0);
                };
                if ui.button("C").clicked() {
                    self.answer = None;
                    self.value = None;
                };
                if ui.button("=").clicked() {};
                if ui.button("+").clicked() {
                    self.click_operator(Operator::Add);
                };
            });
        });
    }
}
