use std::{error::Error, io::Write};

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            _info(format!($($arg)*));
        }
    };
}
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        {
            _warn(format!($($arg)*));
        }
    };
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            _error(format!($($arg)*));
        }
    };
}
#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => {
        {
            _ok(format!($($arg)*));
        }
    };
}

pub(crate) use error;
pub(crate) use info;
pub(crate) use ok;
pub(crate) use warning;

use crate::subprocess;

/**
 * Prints `text` to the terminal as [INFO ].
 */
pub fn _info(text: String) {
    // replace shortened color codes
    let text = text.replace("\x1bc", "\x1b[38;5;27m");
    let text = text.replace("\x1br", "\x1b[1;0m");

    let mut embed_pfx = String::from("\x1b[38;5;240m");
    for _ in 0..subprocess::get_embedding_layers() {
        embed_pfx.push_str("|--> ");
    }

    println!(
        "{embed_pfx}\x1b[38;5;240m[\x1b[38;5;27mINFO \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

/**
 * Prints `text` to the terminal as a [WARN ].
 */
pub fn _warn(text: String) {
    // replace shortened color codes
    let text = text.replace("\x1bc", "\x1b[38;5;214m");
    let text = text.replace("\x1br", "\x1b[1;0m");

    let mut embed_pfx = String::from("\x1b[38;5;240m");
    for _ in 0..subprocess::get_embedding_layers() {
        embed_pfx.push_str("|--> ");
    }

    eprintln!(
        "{embed_pfx}\x1b[38;5;240m[\x1b[38;5;214mWARN \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

/**
 * Prints `text` to the terminal as an [ERROR].
 */
pub fn _error(text: String) {
    // replace shortened color codes
    let text = text.replace("\x1bc", "\x1b[38;5;196m");
    let text = text.replace("\x1br", "\x1b[1;0m");

    let mut embed_pfx = String::from("\x1b[38;5;240m");
    for _ in 0..subprocess::get_embedding_layers() {
        embed_pfx.push_str("|--> ");
    }

    eprintln!(
        "{embed_pfx}\x1b[38;5;240m[\x1b[38;5;196mERROR\x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

pub fn print_error_obj(text: Option<String>, err: Box<dyn Error>) {
    if text.is_some() {
        error!("{}", text.unwrap());
    } else {
        error!("An error occurred. The details are below.");
    }
    error!("{err}");
}

/**
 * Prints `text` to the terminal as [OK   ].
 */
pub fn _ok(text: String) {
    // replace shortened color codes
    let text = text.replace("\x1bc", "\x1b[38;5;40m");
    let text = text.replace("\x1br", "\x1b[1;0m");

    let mut embed_pfx = String::from("\x1b[38;5;240m");
    for _ in 0..subprocess::get_embedding_layers() {
        embed_pfx.push_str("|--> ");
    }

    println!(
        "{embed_pfx}\x1b[38;5;240m[\x1b[38;5;40mOK   \x1b[38;5;240m] \x1b[1;0m{}",
        text
    );
}

/**
 * Asks a question in the terminal.
 * Blocks until the answer is received.
 */
pub fn question(text: String) -> String {
    // replace shortened color codes
    let text = text.replace("\x1bc", "\x1b[38;5;12m");
    let text = text.replace("\x1br", "\x1b[1;0m");

    let mut embed_pfx = String::from("\x1b[38;5;240m");
    for _ in 0..subprocess::get_embedding_layers() {
        embed_pfx.push_str("|--> ");
    }

    print!(
        "{embed_pfx}\x1b[38;5;240m[\x1b[38;5;12mINPUT\x1b[38;5;240m] \x1b[1;0m{text} \x1b[38;5;12m"
    );
    std::io::stdout().flush().ok();

    let mut ans = String::new();

    match std::io::stdin().read_line(&mut ans) {
        Ok(_) => {}
        Err(_) => {
            error!("Failed to read your input. Sending a default value.");
            ans = "INPUTERROR".into();
        }
    };

    print!("\x1b[1;0m");
    std::io::stdout().flush().ok();
    return ans.replace("\n", "");
}
