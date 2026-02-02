mod dwin;

use std::error::Error;

//use dwin::Dwin;

fn main() -> Result<(), Box<dyn Error>> {
    let display_name = std::env::var("DISPLAY")?;

    let wm = dwin::Dwin::new(&display_name)?;

    wm.init()?;
    wm.run();

    Ok(())
}
