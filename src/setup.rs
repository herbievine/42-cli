use figlet_rs::FIGfont;

pub fn setup() {
    print!("\x1B[2J\x1B[1;1H");
    print!("\n\n");

    let small_font = FIGfont::from_file("src/resources/alligator.flf").unwrap();
    let figure = small_font.convert("42  CLI");

    if figure.is_some() {
        println!("{}", figure.unwrap());
    }

    print!("\n\n");
}
