use std::{io, thread::{self, JoinHandle}};

use cursive::{Cursive, CursiveExt, views::Dialog};



pub fn gen_ui() -> JoinHandle<()> {
    thread::spawn(|| {
        let mut ui = Cursive::new();
        let main_view = Dialog::text("Welcome to the DnD character thing!")
            .title("Welcome!")
            .button("Start", |ui| ())
            .button("close", |ui| ui.quit());
    
    ui.add_fullscreen_layer(main_view);
        ui.run();
    })
}