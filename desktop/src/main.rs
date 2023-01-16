use std::collections::HashSet;

use eframe::egui;
use serde::Deserialize;

use crate::app::AppState;

const YOUNGER_FUTHARK: &'static [u8] = include_bytes!("../../data/younger_futhark.csv");

#[derive(Debug, Deserialize)]
struct RunePair {
    rune: String,
    latin: String
}

mod app;
fn load_runes (csv_data: &[u8]) -> Vec<RunePair> {
   let mut vec = Vec::new();

    let mut reader = csv::Reader::from_reader(csv_data);
    for result in reader.deserialize() {
        let record: RunePair = result.unwrap();
        vec.push(record);
    }
    return vec;
}

fn get_characters_from_hashset(rune_pairs: Vec<RunePair>) -> Vec<String> {
    let mut used :HashSet<String>= HashSet::new();
    let mut rune_vector = Vec::new();
    for item in rune_pairs.iter() {
        if used.contains(&item.rune) {
            continue;
        }
        rune_vector.push(item.rune.clone());
        used.insert(item.rune.clone());
    }
    return rune_vector;
}


fn main() {
    let younger_futhark = load_runes(YOUNGER_FUTHARK);
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::vec2(800.0, 150.0));

    eframe::run_native("Othni", native_options, Box::new(|cc| {
        let mut app_state = AppState::new(cc);
        app_state.younger_futhark_characters = get_characters_from_hashset(younger_futhark);

        Box::new(app_state) 
    }));
}

