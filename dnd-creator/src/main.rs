use std::io;
mod stats;
use stats::roll_d;
fn main() -> Result<(), io::Error> {
    println!("{}", roll_d(4));




    Ok(())
}

fn ui() -> Result<(), io::Error> {

    Ok(())
}
