use std::{fs, io::{self, Write}};
use base64::{engine::general_purpose, Engine as _};

pub fn display_image(path: &str, x: u16, y: u16, width: u16, height: u16) -> io::Result<()> {
    let image_data = fs::read(path)?;
    let encoded = general_purpose::STANDARD.encode(&image_data);

    let cmd = format!(
        "\x1b_Gf=100;s={};v={};r=1;c={};x={};y={};a=T;{}\x1b\\",
        width, height, width, x, y, encoded
    );

    io::stdout().write_all(cmd.as_bytes())?;
    io::stdout().flush()?;
    Ok(())
}


