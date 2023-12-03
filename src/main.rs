use std::io::{self, BufRead};

const GRADIENT_STEP: f32 = 2.8;
const LINE_STEP: f32 = 2.2;
const ANGLE: f32 = 45.0; // TODO

fn main() {
    let stdin = io::stdin();
    let mut line_hue = 0.0;

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let colored_line = colorize(&line, line_hue);
                println!("{}", colored_line);
                line_hue = (line_hue + LINE_STEP) % 360.0;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn colorize(line: &str, line_hue: f32) -> String {
    let mut colored_line = String::new();
    let mut char_hue = line_hue;

    for ch in line.chars() {
        let (r, g, b) = hsv_to_rgb(char_hue, 1.0, 1.0); // Full saturation and value
        colored_line.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch));
        char_hue = (char_hue + GRADIENT_STEP) % 360.0;
    }

    colored_line
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match (h as u32) / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
