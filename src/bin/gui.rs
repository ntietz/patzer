use eframe::egui;

pub fn main() {
    let mut options = eframe::NativeOptions::default();
    options.default_theme = eframe::Theme::Light;

    eframe::run_native(
        "Patzer App",
        options,
        Box::new(|_cc| Box::new(PatzerApp::default())),
    );
}

struct PatzerApp {
    name: String,
    age: u32,
}

impl Default for PatzerApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for PatzerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Patzer Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
