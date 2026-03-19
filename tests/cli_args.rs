use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::time::Duration;

mod describe_cli {
    use super::*;

    mod ヘルプとバージョン表示 {
        use super::*;

        #[test]
        fn helpフラグでヘルプメッセージを表示する() {
            Command::cargo_bin("love")
                .unwrap()
                .arg("--help")
                .assert()
                .success()
                .stdout(predicate::str::contains(
                    "A lovely terminal heart animation",
                ));
        }

        #[test]
        fn versionフラグでバージョンを表示する() {
            Command::cargo_bin("love")
                .unwrap()
                .arg("--version")
                .assert()
                .success()
                .stdout(predicate::str::contains("1.2.0"));
        }
    }

    mod 不正なオプションの場合 {
        use super::*;

        #[test]
        fn エラーで終了する() {
            Command::cargo_bin("love")
                .unwrap()
                .arg("--invalid")
                .assert()
                .failure();
        }
    }

    mod メッセージが長すぎる場合 {
        use super::*;

        #[test]
        fn エラーメッセージを表示して終了する() {
            let long_message = "a".repeat(101);
            Command::cargo_bin("love")
                .unwrap()
                .arg("--message")
                .arg(&long_message)
                .assert()
                .failure()
                .stderr(predicate::str::contains("Message too long"));
        }
    }

    mod 有効なオプションで起動した場合 {
        use super::*;

        #[rstest]
        #[case::色オプション(&["--color", "red"])]
        #[case::petiteフラグ(&["--petite"])]
        #[case::短縮メッセージ(&["-m", "Test"])]
        #[case::長形式メッセージ(&["--message", "Test"])]
        #[case::スペース含むメッセージ(&["-m", "Hello World"])]
        #[case::全オプション組み合わせ(&["-m", "Love", "--petite", "--color", "magenta"])]
        fn 引数パースエラーなく起動する(#[case] args: &[&str]) {
            let mut cmd = Command::cargo_bin("love").unwrap();
            for arg in args {
                cmd.arg(arg);
            }
            cmd.timeout(Duration::from_millis(500));
            let output = cmd.output().expect("プロセスの実行に失敗");
            let stderr = String::from_utf8_lossy(&output.stderr);
            // clapの引数パースエラー（exit code 2）が出ていないことを確認
            // CI環境ではターミナル操作のエラーがstderrに出る場合があるため、
            // stderr空チェックではなくexit codeで判定する
            assert_ne!(
                output.status.code(),
                Some(2),
                "引数パースエラー: {}",
                stderr
            );
        }
    }
}
