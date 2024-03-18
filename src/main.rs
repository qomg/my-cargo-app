use ferris_says::say;
use std::io::{stdout, BufWriter};
use eframe::egui::{self, CentralPanel, Context};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    let _ = eframe::run_native(
        "My Egui Window",
        Default::default(),
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

struct MyEguiApp {
    name: String,
    age: u8,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "World".to_owned(),
            age: 30,
        }
    }
    
    fn ui(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Increment").clicked() {
                self.age += 1;
            }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}