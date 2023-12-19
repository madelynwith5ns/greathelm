/**
 * Prints `text` to the terminal as [INFO ].
 */
pub fn info(text: String) {
    println!(
        "\x1b[38;5;240m[\x1b[38;5;27mINFO \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

/**
 * Prints `text` to the terminal as a [WARN ].
 */
pub fn warn(text: String) {
    eprintln!(
        "\x1b[38;5;240m[\x1b[38;5;214mWARN \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

/**
 * Prints `text` to the terminal as an [ERROR].
 */
pub fn error(text: String) {
    eprintln!(
        "\x1b[38;5;240m[\x1b[38;5;196mERROR\x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

/**
 * Prints `text` to the terminal as [OK   ].
 */
pub fn ok(text: String) {
    println!(
        "\x1b[38;5;240m[\x1b[38;5;40mOK   \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}
