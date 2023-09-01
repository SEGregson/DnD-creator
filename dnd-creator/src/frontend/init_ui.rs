use std::{thread::{self, JoinHandle}};

use cursive::{Cursive, CursiveExt, views::{Dialog, SelectView}};

use crate::backend::character_manager::CharacterManager;



pub fn init() -> JoinHandle<()> {
    
    thread::spawn(move || {
        let char_manager = CharacterManager::new();
        let mut ui = Cursive::new();
        let main_view = Dialog::text("Welcome to the DnD character maker!")
            .title("Welcome!")
            .button("Start", |ui| {
                ui.pop_layer();
                main_screen(ui, char_manager);
            })
            .button("close", |ui| ui.quit());
    
        ui.add_layer(main_view);
        ui.run();
    })
}

fn main_screen(ui: &mut Cursive, char_manager: CharacterManager) {
    let character_select = SelectView::<String>::new()
        .add_all_str(char_manager.get_names());


}