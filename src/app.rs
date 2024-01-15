/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CalculatorApp {
    value: f64,
    answer: f64,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            value: 0.0,
            answer: 0.0,
        }
    }
}

impl CalculatorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
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
            ui.heading("calculator");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP)., |ui| {
                ui.label(&format!("{}", self.answer));
            });

            ui.horizontal(|ui| {
                if ui.button("7").clicked() {
                    self.answer = 7.0
                };
                if ui.button("8").clicked() {
                    self.answer = 8.0
                };
                if ui.button("9").clicked() {
                    self.answer = 9.0
                };
                if ui.button("/").clicked() {};
            });

            ui.horizontal(|ui| {
                if ui.button("4").clicked() {
                    self.answer = 4.0
                };
                if ui.button("5").clicked() {
                    self.answer = 5.0
                };
                if ui.button("6").clicked() {
                    self.answer = 6.0
                };
                if ui.button("x").clicked() {};
            });

            ui.horizontal(|ui| {
                if ui.button("1").clicked() {
                    self.answer = 1.0
                };
                if ui.button("2").clicked() {
                    self.answer = 2.0
                };
                if ui.button("3").clicked() {
                    self.answer = 3.0
                };
                if ui.button("-").clicked() {};
            });

            ui.horizontal(|ui| {
                if ui.button("0").clicked() {
                    self.answer = 0.0
                };
                if ui.button("C").clicked() {
                    self.answer = 0.0
                };
                if ui.button("=").clicked() {};
                if ui.button("+").clicked() {};
            });
        });
    }
}
