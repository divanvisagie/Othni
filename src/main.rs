use eframe::egui;
use egui::{FontData, FontFamily, FontDefinitions};


//create a string array of runes
const YOUNGER_FUTHARK: [&str; 16] = ["ᚠ","ᚢ","ᚦ", "ᚬ", "ᚱ", "ᚴ", "ᚼ", "ᚾ", "ᛁ", "ᛅ", "ᛋ", "ᛏ", "ᛒ", "ᛘ", "ᛚ", "ᛦ"];
const LATIN_REP: [&str; 16] = ["f", "u", "þ", "ą", "r", "k", "h", "n", "i", "a", "s", "t", "b", "m", "I", "R"];
const FONT_KEY: &str = "my_font";

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(800.0, 150.0));
    eframe::run_native("Othni", native_options, Box::new(|cc| Box::new(AppState::new(cc))));
}

fn load_compatible_font () -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    // Install font to  support non-latin characters:
    fonts.font_data.insert(FONT_KEY.to_owned(),
    FontData::from_static(include_bytes!("../fonts/segoe ui historic.ttf"))); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, FONT_KEY.to_owned());

    // Put my font as last fallback for monospace:
    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push(FONT_KEY.to_owned());
    fonts
}

#[derive(Default)]
struct AppState {
    latin_string: String,
    younger_futhark_string: String
}

impl AppState {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        // load font definition from File

        let fonts =  load_compatible_font();

        cc.egui_ctx.set_fonts(fonts);
        Self::default()
    }
}

impl eframe::App for AppState {

   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
    
        ui.heading(self.latin_string.as_str());
        ui.heading(self.younger_futhark_string.as_str());

        ui.horizontal(|ui| {
            ui.label("Latin string:");
            ui.text_edit_singleline(&mut self.latin_string);
        });
        ui.horizontal(|ui| {
            ui.label("Younger Futhark string:");
            ui.text_edit_singleline(&mut self.younger_futhark_string);
        });

        ui.horizontal(|ui| {
            // for each rune in RUNES
            for rune in YOUNGER_FUTHARK.iter() {
                // create a button
                if ui.button(*rune).clicked() {
                    // add character to younger_futhark_string
                    self.younger_futhark_string.push_str(rune);
                    self.latin_string.push_str(LATIN_REP[YOUNGER_FUTHARK.iter().position(|&r| r == *rune).unwrap()]);
                }
            }
        });

       });
   }
}