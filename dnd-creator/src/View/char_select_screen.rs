use cursive::{Cursive, views::SelectView};

use crate::Model::character_manager::CharacterManager;

pub fn select_char_view(ui: &mut Cursive, char_manager: &mut CharacterManager) {
    let mut select_char = SelectView::<String>::new();
    select_char.add_all(char_manager.get_names());
}