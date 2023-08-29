use std::io;
mod stats;
use stats::roll_d;

mod frontend;
use crate::frontend::draw::gen_ui;
fn main() -> Result<(), io::Error> {
    


    let ui = gen_ui();
    println!("{}", roll_d(4));

    ui.join().unwrap();
    Ok(())
}


