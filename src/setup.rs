use figlet_rs::FIGfont;

pub fn setup() {
    print!("\x1B[2J\x1B[1;1H");
    print!("\n");

    let font_bytes = include_bytes!("resources/alligator.flf");

    match FIGfont::from_content(&String::from_utf8_lossy(font_bytes)) {
        Ok(font) => {
            let figure = font.convert("42  CLI");
            if figure.is_some() {
                println!("{}", figure.unwrap());
            }
        }
        _ => {
            println!("42 CLI");
        }
    }

    print!("\n");
}
