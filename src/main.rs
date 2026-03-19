use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute,
    style::{ResetColor, SetForegroundColor},
    terminal::{size, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use std::io::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{atomic, Arc};
use std::thread;
use std::time::Duration;
use tolove_ru::{heart_sizes, is_in_love, parse_color, HeartConfig, ABOUT_MESSAGE};

#[derive(Parser, Clone)]
#[clap(author, version)]
#[command(about = ABOUT_MESSAGE)]
struct Options {
    #[clap(short, long, value_parser = tolove_ru::validate_message)]
    message: Option<String>,

    #[clap(long)]
    petite: bool,

    #[clap(long, default_value = "white")]
    color: String,
}

impl From<&Options> for HeartConfig {
    fn from(options: &Options) -> Self {
        HeartConfig {
            message: options.message.clone(),
            petite: options.petite,
            color: options.color.clone(),
        }
    }
}

fn main() -> Result<()> {
    // コマンドライン引数の取り扱い
    let options = Options::parse();
    let config = HeartConfig::from(&options);

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
        if draw_love_row_with_message(y, &config)? {
            break;
        }
        thread::sleep(Duration::from_millis(300));
        y += 1;
    }

    execute!(stdout(), RestorePosition, Show, LeaveAlternateScreen)?;
    Ok(())
}

fn draw_love_row_with_message(y: i32, config: &HeartConfig) -> Result<bool> {
    let (heart_size, half_size) = heart_sizes(config);

    let message = match config.message {
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
    execute!(stdout(), SetForegroundColor(parse_color(&config.color)))?;

    loop {
        print!("{}", if is_in_love(x, y, config) { "vv" } else { "  " });
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
