use std::io;

mod Model;
mod Controller;
mod View;

use Model::character_manager::CharacterManager;
use View::index_view::index;

fn main() -> Result<(), io::Error> {
    
    let manager = CharacterManager::new();

    let ui = index();

    ui.join().unwrap();
    Ok(())
}


