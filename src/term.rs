pub fn info(text: String) {
    println!(
        "\x1b[38;5;240m[\x1b[38;5;27mINFO \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

pub fn warn(text: String) {
    eprintln!(
        "\x1b[38;5;240m[\x1b[38;5;214mWARN \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

pub fn error(text: String) {
    eprintln!(
        "\x1b[38;5;240m[\x1b[38;5;196mERROR\x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

pub fn ok(text: String) {
    println!(
        "\x1b[38;5;240m[\x1b[38;5;40mOK   \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}
