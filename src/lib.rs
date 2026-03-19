use crossterm::style::Color;

// ハートの描画領域
pub const HEART_SIZE_L: i32 = 20;
pub const HEART_SIZE_S: i32 = 10;

pub const ABOUT_MESSAGE: &str = "
┌---------------------------------------------------------------------------┐
|   vvvvvv  vvvvvvv      A lovely terminal heart animation.                 |
| vvvvvvvvvvvvvvvvvv                                                        |
| vvvvvvvvvvvvvvvvvvv    Watch the heart float up...                        |
| vvvvvvvvvvvvvvvvvv     Add your message inside...                         |
|   vvvvvvvvvvvvvv       And share the love!                                |
|     vvvvvvvvvv                                                            |
|       vvvvvv           Type 'love --help' for more details                |
|         vv                                                                |
└---------------------------------------------------------------------------┘";

/// CLIフレームワーク非依存のハート設定
pub struct HeartConfig {
    pub message: Option<String>,
    pub petite: bool,
    pub color: String,
}

/// 入力をサニタイズし、制御文字やエスケープシーケンスを除去する
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|&c| {
            // 印字可能文字、スペース、タブ、改行のみ許可
            // 制御文字 (0x00-0x1F, 0x7F-0x9F) を除去
            let code = c as u32;
            (0x20..0x7F).contains(&code) || c == '\t' || c == '\n'
        })
        .collect()
}

/// メッセージ入力のバリデーションとサニタイズ
pub fn validate_message(s: &str) -> Result<String, String> {
    const MAX_MESSAGE_LENGTH: usize = 100;

    if s.len() > MAX_MESSAGE_LENGTH {
        return Err(format!(
            "Message too long (max {} characters)",
            MAX_MESSAGE_LENGTH
        ));
    }

    Ok(sanitize_input(s))
}

/// 色名文字列を対応するColorに変換する
pub fn parse_color(color_str: &str) -> Color {
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

/// 座標がハート形状の内部にあるかを判定する
pub fn is_in_love(x: i32, y: i32, config: &HeartConfig) -> bool {
    let (heart_size, _) = heart_sizes(config);

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

/// ハートのサイズを返す (幅, 半幅)
pub fn heart_sizes(config: &HeartConfig) -> (i32, i32) {
    if config.petite {
        return (HEART_SIZE_S, HEART_SIZE_S / 2);
    };

    (HEART_SIZE_L, HEART_SIZE_L / 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::Color;
    use rstest::rstest;

    // テスト用のヘルパー: デフォルトのHeartConfigを生成
    fn default_config() -> HeartConfig {
        HeartConfig {
            message: None,
            petite: false,
            color: "white".to_string(),
        }
    }

    fn petite_config() -> HeartConfig {
        HeartConfig {
            message: None,
            petite: true,
            color: "white".to_string(),
        }
    }

    // ================================================================
    // sanitize_input: 入力サニタイズの仕様
    // ================================================================

    mod describe_sanitize_input {
        use super::*;

        mod 印字可能なascii文字を受け取った場合 {
            use super::*;

            #[test]
            fn そのまま返す() {
                assert_eq!(sanitize_input("Hello World"), "Hello World");
            }
        }

        mod 制御文字を受け取った場合 {
            use super::*;

            #[rstest]
            #[case::ansi_esc("\x1b[31mRed\x1b[0m", "[31mRed[0m")]
            #[case::ベル文字("\x07Bell", "Bell")]
            #[case::ヌルバイト("Hello\x00World", "HelloWorld")]
            #[case::del文字("Text\x7FMore", "TextMore")]
            #[case::c1制御コード("Test\u{009B}More", "TestMore")]
            fn 除去する(#[case] input: &str, #[case] expected: &str) {
                assert_eq!(sanitize_input(input), expected);
            }
        }

        mod ターミナルインジェクション攻撃を受けた場合 {
            use super::*;

            #[rstest]
            #[case::タイトルインジェクション("\x1b]0;Evil\x07", "]0;Evil")]
            #[case::画面クリアインジェクション("\x1b[2J\x1b[H", "[2J[H")]
            #[case::複合攻撃("\x1b[31m\x00\x07Evil\x1b[0m", "[31mEvil[0m")]
            fn 制御文字のみ除去して無害化する(
                #[case] input: &str,
                #[case] expected: &str,
            ) {
                assert_eq!(sanitize_input(input), expected);
            }
        }

        mod タブや改行を受け取った場合 {
            use super::*;

            #[test]
            fn 保持する() {
                assert_eq!(sanitize_input("Line1\tTab\nLine2"), "Line1\tTab\nLine2");
            }
        }

        mod unicodeや絵文字を受け取った場合 {
            use super::*;

            #[test]
            fn ascii範囲外のため除去する() {
                assert_eq!(sanitize_input("❤️💜"), "");
            }
        }

        mod 空文字列を受け取った場合 {
            use super::*;

            #[test]
            fn 空文字列を返す() {
                assert_eq!(sanitize_input(""), "");
            }
        }
    }

    // ================================================================
    // validate_message: メッセージバリデーションの仕様
    // ================================================================

    mod describe_validate_message {
        use super::*;

        mod 正常なメッセージの場合 {
            use super::*;

            #[test]
            fn サニタイズ済みの文字列を返す() {
                let result = validate_message("I love you");
                assert_eq!(result.unwrap(), "I love you");
            }
        }

        mod メッセージの長さに関する境界値 {
            use super::*;

            #[test]
            fn 最大長100文字は受け付ける() {
                let input = "a".repeat(100);
                assert!(validate_message(&input).is_ok());
            }

            #[rstest]
            #[case::境界値超過(101)]
            #[case::dos攻撃(1000)]
            fn 最大長を超えるとエラーを返す(#[case] length: usize) {
                let input = "a".repeat(length);
                let result = validate_message(&input);
                assert!(result.is_err());
                assert!(result.unwrap_err().contains("Message too long"));
            }
        }

        mod エスケープシーケンスを含む場合 {
            use super::*;

            #[test]
            fn サニタイズして受け付ける() {
                let result = validate_message("Hello\x1b[31mWorld");
                assert_eq!(result.unwrap(), "Hello[31mWorld");
            }
        }

        mod 空メッセージの場合 {
            use super::*;

            #[test]
            fn 空文字列として受け付ける() {
                assert_eq!(validate_message("").unwrap(), "");
            }
        }

        mod unicodeメッセージの場合 {
            use super::*;

            #[test]
            fn ascii範囲外の文字を除去して受け付ける() {
                // セキュリティのため、ASCII範囲外の文字は除去される
                assert_eq!(validate_message("愛してる💜").unwrap(), "");
            }
        }
    }

    // ================================================================
    // parse_color: 色パースの仕様
    // ================================================================

    mod describe_parse_color {
        use super::*;

        mod 有効な色名の場合 {
            use super::*;

            #[rstest]
            #[case::赤("red", Color::Red)]
            #[case::緑("green", Color::Green)]
            #[case::青("blue", Color::Blue)]
            #[case::黄("yellow", Color::Yellow)]
            #[case::マゼンタ("magenta", Color::Magenta)]
            #[case::シアン("cyan", Color::Cyan)]
            #[case::白("white", Color::White)]
            fn 対応するcolorを返す(#[case] input: &str, #[case] expected: Color) {
                assert_eq!(parse_color(input), expected);
            }
        }

        mod 無効な色名の場合 {
            use super::*;

            #[rstest]
            #[case::不明な文字列("invalid")]
            #[case::空文字列("")]
            #[case::大文字("RED")]
            fn デフォルトの白色を返す(#[case] input: &str) {
                assert_eq!(parse_color(input), Color::White);
            }
        }
    }

    // ================================================================
    // heart_sizes: ハートサイズの仕様
    // ================================================================

    mod describe_heart_sizes {
        use super::*;

        mod 通常モードの場合 {
            use super::*;

            #[test]
            fn 幅20_半幅10を返す() {
                let (width, half) = heart_sizes(&default_config());
                assert_eq!(width, 20);
                assert_eq!(half, 10);
            }
        }

        mod petiteモードの場合 {
            use super::*;

            #[test]
            fn 幅10_半幅5を返す() {
                let (width, half) = heart_sizes(&petite_config());
                assert_eq!(width, 10);
                assert_eq!(half, 5);
            }
        }
    }

    // ================================================================
    // is_in_love: ハート形状判定の仕様
    // ================================================================

    mod describe_is_in_love {
        use super::*;

        mod ハート内部の座標の場合 {
            use super::*;

            #[test]
            fn 通常サイズの中心はtrueを返す() {
                assert!(is_in_love(10, 10, &default_config()));
            }

            #[test]
            fn petiteサイズの中心はtrueを返す() {
                assert!(is_in_love(5, 5, &petite_config()));
            }
        }

        mod ハート外部の座標の場合 {
            use super::*;

            #[rstest]
            #[case::左外(-5, 10)]
            #[case::右外(25, 10)]
            #[case::上外(10, 25)]
            #[case::下外(10, -5)]
            #[case::左上角(0, 0)]
            #[case::右下角(20, 20)]
            fn falseを返す(#[case] x: i32, #[case] y: i32) {
                assert!(!is_in_love(x, y, &default_config()));
            }
        }

        mod ハート境界付近の座標の場合 {
            use super::*;

            #[rstest]
            #[case::左上寄り(5, 15)]
            #[case::右上寄り(15, 15)]
            #[case::下部(10, 5)]
            fn 境界の外側はfalseを返す(#[case] x: i32, #[case] y: i32) {
                assert!(!is_in_love(x, y, &default_config()));
            }
        }
    }
}
