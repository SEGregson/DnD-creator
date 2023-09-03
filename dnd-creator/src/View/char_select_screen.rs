use std::sync::Arc;

use cursive::{Cursive, views::{SelectView, Dialog, LinearLayout, TextView, TextArea, Button, DummyView}, view::Resizable};

use crate::Model::character_manager::CharacterManager;

pub fn select_char_view(ui: &mut Cursive, char_manager: Arc<CharacterManager>) {
    
    let select_char_view = update_list(SelectView::<String>::new(), char_manager);    
    let sample_view = Dialog::text("this is a placeholder for the viewing of char sheets")
        .title("placeholder");

    ui.add_layer(
        LinearLayout::horizontal()
            .child(Dialog::around(select_char_view)
                .title("characters"))
            .child(DummyView)
            .child(sample_view)
        
    );
}

fn update_list(mut select_char: SelectView, char_manager: Arc<CharacterManager>) -> SelectView {
    let local = char_manager.clone();
    for i in local.get_names() {
        select_char.add_item_str(i);
    }
    select_char
}