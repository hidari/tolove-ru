mod shape;
use shape::*;

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{size, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use std::io::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{atomic, Arc};
use std::thread;
use std::time::Duration;

#[derive(Parser, Clone)]
#[clap(author, version)]
#[command(about = ABOUT_MESSAGE)]
struct Options {
    #[clap(short, long, value_parser = validate_message)]
    message: Option<String>,

    #[clap(long)]
    petite: bool,

    #[clap(long, default_value = "white")]
    color: String,
}

/// Validates and sanitizes the message input
fn validate_message(s: &str) -> std::result::Result<String, String> {
    const MAX_MESSAGE_LENGTH: usize = 100;

    if s.len() > MAX_MESSAGE_LENGTH {
        return Err(format!(
            "Message too long (max {} characters)",
            MAX_MESSAGE_LENGTH
        ));
    }

    Ok(sanitize_input(s))
}

/// Sanitizes input by removing control characters and escape sequences
fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|&c| {
            // Allow printable characters, spaces, tabs, and common punctuation
            // Filter out control characters (0x00-0x1F, 0x7F-0x9F)
            let code = c as u32;
            (0x20..0x7F).contains(&code) || c == '\t' || c == '\n'
        })
        .collect()
}

/// Parses a color string and returns the corresponding Color
fn parse_color(color_str: &str) -> Color {
    match color_str {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        _ => Color::White,
    }
}

fn main() -> Result<()> {
    // コマンドライン引数の取り扱い
    let options = Options::parse();

    execute!(stdout(), EnterAlternateScreen, Hide, SavePosition)?;

    // Ctrl-Cを受け付けるハンドラの設定
    let running = Arc::new(AtomicBool::new(true));
    let state = Arc::clone(&running);
    ctrlc::set_handler(move || {
        state.store(false, atomic::Ordering::Relaxed);
    })
    .expect("Setting Ctrl-C handler failed.");

    let (_, rows) = size()?;
    execute!(stdout(), MoveTo(0, rows))?;

    let mut y = 0;
    while running.load(atomic::Ordering::Relaxed) {
        if draw_love_row_with_message(y, &options)? {
            break;
        }
        thread::sleep(Duration::from_millis(300));
        y += 1;
    }

    execute!(stdout(), RestorePosition, Show, LeaveAlternateScreen)?;
    Ok(())
}

fn draw_love_row_with_message(y: i32, options: &Options) -> Result<bool> {
    let (heart_size, half_size) = heart_sizes(options);

    let message = match options.message {
        Some(ref string) => format!(" {} ", string),
        None => "".to_string(),
    };
    let message_indent = (half_size - (message.len() / 4) as i32) - 1;

    let (cols, rows) = size()?;
    let rows = rows as i32;
    let cols = cols as i32;

    let mut x = 0;

    let indent = ((cols / 2) - half_size) - 10;
    for _ in 0..indent {
        print!(" ");
    }

    // 色を設定
    execute!(stdout(), SetForegroundColor(parse_color(&options.color)))?;

    loop {
        print!(
            "{}",
            if is_in_love(x, y, options) {
                "vv"
            } else {
                "  "
            }
        );
        if y == half_size - 1 && x == message_indent {
            print!("{}", message);
            x += (message.len() / 2) as i32;
        }

        if x >= heart_size {
            break;
        }

        x += 1;
    }

    execute!(stdout(), ResetColor)?;
    println!();

    if y < rows + heart_size {
        Ok(false)
    } else {
        Ok(true)
    }
}

fn is_in_love(x: i32, y: i32, options: &Options) -> bool {
    let (heart_size, _) = heart_sizes(options);

    let width = 2.2;
    let height = 3.0;
    let heart_coefficient = 0.7;

    let check_x = ((x as f64 / heart_size as f64) - 0.5) * width;
    let check_y = (((heart_size - y) as f64 / heart_size as f64) - 0.4) * height;

    let top_y: f64;
    let bottom_y: f64;

    if check_x >= 0.0 {
        top_y = (1.0 - (check_x * check_x)).sqrt() + (heart_coefficient * check_x.sqrt());
        bottom_y = -(1.0 - (check_x * check_x)).sqrt() + (heart_coefficient * check_x.sqrt());
    } else {
        top_y = (1.0 - (check_x * check_x)).sqrt() + (heart_coefficient * (-check_x).sqrt());
        bottom_y = -(1.0 - (check_x * check_x)).sqrt() + (heart_coefficient * (-check_x).sqrt());
    }

    (bottom_y <= check_y) && (check_y <= top_y)
}

fn heart_sizes(options: &Options) -> (i32, i32) {
    if options.petite {
        return (HEART_SIZE_S, HEART_SIZE_S / 2);
    };

    (HEART_SIZE_L, HEART_SIZE_L / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // Phase 1: セキュリティテスト
    // ========================================

    // sanitize_input のテスト (12個)

    #[test]
    fn test_sanitize_normal_ascii() {
        let input = "Hello World";
        let result = sanitize_input(input);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_sanitize_ansi_escape_removed() {
        let input = "\x1b[31mRed\x1b[0m";
        let result = sanitize_input(input);
        assert_eq!(result, "[31mRed[0m");
    }

    #[test]
    fn test_sanitize_bell_character_removed() {
        let input = "\x07Bell";
        let result = sanitize_input(input);
        assert_eq!(result, "Bell");
    }

    #[test]
    fn test_sanitize_null_byte_removed() {
        let input = "Hello\x00World";
        let result = sanitize_input(input);
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_sanitize_terminal_title_injection() {
        let input = "\x1b]0;Evil\x07";
        let result = sanitize_input(input);
        assert_eq!(result, "]0;Evil");
    }

    #[test]
    fn test_sanitize_screen_clear_injection() {
        let input = "\x1b[2J\x1b[H";
        let result = sanitize_input(input);
        assert_eq!(result, "[2J[H");
    }

    #[test]
    fn test_sanitize_unicode_emoji_removed() {
        let input = "❤️💜";
        let result = sanitize_input(input);
        // セキュリティのため、ASCII範囲外の文字（絵文字含む）は除去される
        assert_eq!(result, "");
    }

    #[test]
    fn test_sanitize_tab_and_newline_preserved() {
        let input = "Line1\tTab\nLine2";
        let result = sanitize_input(input);
        assert_eq!(result, "Line1\tTab\nLine2");
    }

    #[test]
    fn test_sanitize_delete_character_removed() {
        let input = "Text\x7FMore";
        let result = sanitize_input(input);
        assert_eq!(result, "TextMore");
    }

    #[test]
    fn test_sanitize_c1_control_codes_removed() {
        let input = "Test\u{009B}More";
        let result = sanitize_input(input);
        assert_eq!(result, "TestMore");
    }

    #[test]
    fn test_sanitize_mixed_attack() {
        let input = "\x1b[31m\x00\x07Evil\x1b[0m";
        let result = sanitize_input(input);
        assert_eq!(result, "[31mEvil[0m");
    }

    #[test]
    fn test_sanitize_empty_string() {
        let input = "";
        let result = sanitize_input(input);
        assert_eq!(result, "");
    }

    // validate_message のテスト (7個)

    #[test]
    fn test_validate_normal_message() {
        let input = "I love you";
        let result = validate_message(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "I love you");
    }

    #[test]
    fn test_validate_message_max_length() {
        let input = "a".repeat(100);
        let result = validate_message(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_message_too_long() {
        let input = "a".repeat(101);
        let result = validate_message(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Message too long"));
    }

    #[test]
    fn test_validate_dos_attack_billion_chars() {
        // 実際に10億文字作るとメモリを消費するので、制限値を超える文字列でテスト
        let input = "a".repeat(1000);
        let result = validate_message(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_with_escape_sequences() {
        let input = "Hello\x1b[31mWorld";
        let result = validate_message(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello[31mWorld");
    }

    #[test]
    fn test_validate_empty_message() {
        let input = "";
        let result = validate_message(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_validate_unicode_message() {
        let input = "愛してる💜";
        let result = validate_message(input);
        assert!(result.is_ok());
        // セキュリティのため、ASCII範囲外の文字は除去される
        assert_eq!(result.unwrap(), "");
    }

    // ========================================
    // Phase 2: コアロジックテスト
    // ========================================

    // heart_sizes のテスト (2個)

    #[test]
    fn test_heart_sizes_large() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        let (heart_size, half_size) = heart_sizes(&options);
        assert_eq!(heart_size, HEART_SIZE_L);
        assert_eq!(half_size, HEART_SIZE_L / 2);
        assert_eq!(heart_size, 20);
        assert_eq!(half_size, 10);
    }

    #[test]
    fn test_heart_sizes_petite() {
        let options = Options {
            message: None,
            petite: true,
            color: "white".to_string(),
        };
        let (heart_size, half_size) = heart_sizes(&options);
        assert_eq!(heart_size, HEART_SIZE_S);
        assert_eq!(half_size, HEART_SIZE_S / 2);
        assert_eq!(heart_size, 10);
        assert_eq!(half_size, 5);
    }

    // is_in_love のテスト (10個)

    #[test]
    fn test_is_in_love_center() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // ハート中心部は内側にあるはず
        assert!(is_in_love(10, 10, &options));
    }

    #[test]
    fn test_is_in_love_near_top_left() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // 左上寄りの座標（実際の計算では外側と判定される）
        assert!(!is_in_love(5, 15, &options));
    }

    #[test]
    fn test_is_in_love_near_top_right() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // 右上寄りの座標（実際の計算では外側と判定される）
        assert!(!is_in_love(15, 15, &options));
    }

    #[test]
    fn test_is_in_love_bottom_area() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // 下部の座標（実際の計算では外側と判定される）
        assert!(!is_in_love(10, 5, &options));
    }

    #[test]
    fn test_is_in_love_outside_left() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // 左外側（完全に範囲外）
        assert!(!is_in_love(-5, 10, &options));
    }

    #[test]
    fn test_is_in_love_outside_right() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // 右外側（完全に範囲外）
        assert!(!is_in_love(25, 10, &options));
    }

    #[test]
    fn test_is_in_love_outside_top() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // 上外側
        assert!(!is_in_love(10, 25, &options));
    }

    #[test]
    fn test_is_in_love_outside_bottom() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // 下外側
        assert!(!is_in_love(10, -5, &options));
    }

    #[test]
    fn test_is_in_love_petite_center() {
        let options = Options {
            message: None,
            petite: true,
            color: "white".to_string(),
        };
        // 小ハート中心
        assert!(is_in_love(5, 5, &options));
    }

    #[test]
    fn test_is_in_love_boundary_cases() {
        let options = Options {
            message: None,
            petite: false,
            color: "white".to_string(),
        };
        // 境界値テスト: (0, 0) は範囲外のはず
        assert!(!is_in_love(0, 0, &options));
        // (20, 20) も範囲外のはず
        assert!(!is_in_love(20, 20, &options));
    }

    // ========================================
    // Phase 3: 色変換テスト
    // ========================================

    #[test]
    fn test_parse_color_red() {
        let color = parse_color("red");
        assert_eq!(color, Color::Red);
    }

    #[test]
    fn test_parse_color_green() {
        let color = parse_color("green");
        assert_eq!(color, Color::Green);
    }

    #[test]
    fn test_parse_color_blue() {
        let color = parse_color("blue");
        assert_eq!(color, Color::Blue);
    }

    #[test]
    fn test_parse_color_yellow() {
        let color = parse_color("yellow");
        assert_eq!(color, Color::Yellow);
    }

    #[test]
    fn test_parse_color_magenta() {
        let color = parse_color("magenta");
        assert_eq!(color, Color::Magenta);
    }

    #[test]
    fn test_parse_color_cyan() {
        let color = parse_color("cyan");
        assert_eq!(color, Color::Cyan);
    }

    #[test]
    fn test_parse_color_white() {
        let color = parse_color("white");
        assert_eq!(color, Color::White);
    }

    #[test]
    fn test_parse_color_invalid() {
        let color = parse_color("invalid");
        // 不正な値はデフォルトの白色になる
        assert_eq!(color, Color::White);
    }

    #[test]
    fn test_parse_color_empty() {
        let color = parse_color("");
        // 空文字列もデフォルトの白色になる
        assert_eq!(color, Color::White);
    }

    #[test]
    fn test_parse_color_case_sensitive() {
        let color = parse_color("RED");
        // 大文字は不正な値として扱われ、デフォルトの白色になる
        assert_eq!(color, Color::White);
    }
}
