use console::Style;

pub(crate) fn running(message: &str) {
    status(
        message,
        Style::new().blue().bold(),
        Style::new().white().bold(),
    )
}

pub(crate) fn success(message: &str) {
    status(
        message,
        Style::new().green().bold(),
        Style::new().white().bold(),
    )
}

fn status(message: &str, head: Style, tail: Style) {
    println!("{} {}", head.apply_to(">>>"), tail.apply_to(message))
}
