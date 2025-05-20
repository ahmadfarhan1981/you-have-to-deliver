pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn underline(s: &str) -> String {
    format!("\x1b[4m{}\x1b[0m", s)
}

pub fn dim(s: &str) -> String {
    format!("\x1b[2m{}\x1b[0m", s)
}

pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

pub fn yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

pub fn magenta(s: &str) -> String {
    format!("\x1b[35m{}\x1b[0m", s)
}

pub fn cyan(s: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", s)
}

pub fn italic(s: &str) -> String {
    format!("\x1b[3m{}\x1b[0m", s)
}
