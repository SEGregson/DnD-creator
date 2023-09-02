use std::{thread::{self, JoinHandle}};

use cursive::{Cursive, CursiveExt, views::Dialog};

use crate::Model::character_manager::CharacterManager;

use super::char_select_screen::select_char_view;


pub fn index(char_manager: CharacterManager) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut ui = Cursive::new();
        let main_view = Dialog::text("Welcome to the DnD character maker!")
            .title("Welcome!")
            .button("Start", |ui| {
                ui.pop_layer();
                select_char_view(ui, &mut char_manager);
            })
            .button("close", |ui| ui.quit());
    
        ui.add_layer(main_view);
        ui.run();
    })
}