pub fn info(text: String) {
    println!("\x1b[38;5;27mⓘ  \x1b[1;0m{}", text);
}

pub fn warn(text: String) {
    eprintln!("\x1b[38;5;214m⚠  \x1b[1;0m{}", text);
}

pub fn error(text: String) {
    eprintln!("\x1b[38;5;196m⛌  \x1b[1;0m{}", text);
}

pub fn ok(text: String) {
    println!("\x1b[38;5;40m✔  \x1b[1;0m{}", text);
}
