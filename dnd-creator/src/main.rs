use std::{io, sync::Arc};

mod Model;
mod Controller;
mod View;

use Model::character_manager::CharacterManager;
use View::index_view::index;

fn main() -> Result<(), io::Error> {
    
    let manager = Arc::from(CharacterManager::new());

    index(manager).join().unwrap();

    Ok(())
}


