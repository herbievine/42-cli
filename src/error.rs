use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn display_error<S: Into<String>>(message: S, command: S, exit: bool) {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    let error_message = format!(
        "\nStacktrace:\n{}\nCommand:\n{}\n",
        message.into(),
        command.into()
    );

    stderr
        .set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Red))
                .set_bg(Some(Color::Black)),
        )
        .unwrap();

    match writeln!(&mut stderr, "{}", error_message).unwrap() {
        _ => {
            if exit {
                std::process::exit(1)
            }
        }
    }
}
