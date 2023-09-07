use std::sync::Arc;

use cursive::{Cursive, views::{SelectView, Dialog, LinearLayout, Button, DummyView}, view::Nameable};

use crate::Model::character_manager::CharacterManager;

pub fn select_char_view(ui: &mut Cursive, char_manager: Arc<CharacterManager>) {
    fn view_sheet(ui: &mut Cursive) {}
    fn edit_sheet(ui: &mut Cursive) {}
    fn delete_sheet(ui: &mut Cursive) {}
    fn new_sheet(ui: &mut Cursive) {}
    
    let select_char_view = update_list(SelectView::<String>::new(), char_manager)
        .with_name("select_char_view");

    let control_view = LinearLayout::vertical()
        .child(Button::new("view", view_sheet))
        .child(Button::new("edit", edit_sheet))
        .child(Button::new("delete", delete_sheet))
        .child(DummyView)
        .child(Button::new("new", new_sheet));

    ui.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(control_view)
                .child(select_char_view)
            )
            .title("characters")
        );
                
        
    
}

fn update_list(mut select_char: SelectView, char_manager: Arc<CharacterManager>) -> SelectView {
    let local = char_manager.clone();
    for i in local.get_names() {
        select_char.add_item_str(i);
    }
    select_char
}