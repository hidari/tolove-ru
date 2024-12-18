mod shape;
use shape::*;

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute,
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
    #[clap(short, long)]
    message: Option<String>,

    #[clap(long)]
    petite: bool,
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

    let (_, rows) = match size() {
        Ok((cols, rows)) => (cols, rows),
        Err(e) => panic!("Error getting terminal size: {:?}", e),
    };
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
    let (heart_size, half_size) = heart_sizes(&options);

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

    loop {
        print!("{}", if is_in_love(x, y, &options) { "vv" } else { "  " });
        if y == half_size - 1 {
            if x == message_indent {
                print!("{}", message);
                x += (message.len() / 2) as i32;
            }
        }

        if x >= heart_size {
            break;
        }

        x += 1;
    }

    println!();

    if y < rows + heart_size {
        Ok(false)
    } else {
        Ok(true)
    }
}

fn is_in_love(x: i32, y: i32, options: &Options) -> bool {
    let (heart_size, _) = heart_sizes(&options);

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

    if (bottom_y <= check_y) && (check_y <= top_y) {
        true
    } else {
        false
    }
}

fn heart_sizes(options: &Options) -> (i32, i32){
    if options.petite {
       return (HEART_SIZE_S, HEART_SIZE_S / 2)
    };

    (HEART_SIZE_L, HEART_SIZE_L / 2)
}
