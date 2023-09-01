use std::io;

mod backend;
use backend::stats::roll_d;

mod frontend;
use crate::frontend::init_ui::init;
fn main() -> Result<(), io::Error> {
    


    let ui = init();

    ui.join().unwrap();
    Ok(())
}


