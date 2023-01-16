
use eframe::egui;
use egui::{FontData, FontFamily, FontDefinitions};

const FONT_KEY: &str = "Historic";

fn load_compatible_font () -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    // Install font to support non-latin characters:
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
pub struct AppState {
    pub younger_futhark_characters: Vec<String>,
    pub latin_string: String,
    runic_string: String
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let fonts =  load_compatible_font();
        cc.egui_ctx.set_fonts(fonts);
        Self::default()
    }
}

impl eframe::App for AppState {

   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
    
        ui.heading(self.latin_string.as_str());
        ui.heading(self.runic_string.as_str());

        ui.horizontal(|ui| {
            ui.label("Latin string:");
            ui.text_edit_singleline(&mut self.latin_string);
        });
        ui.horizontal(|ui| {
            ui.label("Runic string:");
            ui.text_edit_singleline(&mut self.runic_string);
        });

        ui.horizontal(|ui| {
            // for each rune in RUNES
            for rune in self.younger_futhark_characters.iter() {
                // create a button
                if ui.button(rune).clicked() {
                    // add character to younger_futhark_string
                    self.runic_string.push_str(rune);
                  
                }
            }
        });

       });
   }
}