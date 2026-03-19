#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;
use rstest::rstest;
use std::time::Duration;

mod describe_エラーハンドリング {
    use super::*;

    mod メッセージが最大長を超える場合 {
        use super::*;

        #[rstest]
        #[case::境界値超過(101)]
        #[case::dos攻撃(1000)]
        fn エラーメッセージを表示して終了する(#[case] length: usize) {
            let msg = "a".repeat(length);
            Command::cargo_bin("love")
                .unwrap()
                .arg("--message")
                .arg(&msg)
                .assert()
                .failure()
                .stderr(predicate::str::contains("Message too long"))
                .stderr(predicate::str::contains("max 100 characters"));
        }
    }

    mod エスケープシーケンスを含むメッセージの場合 {
        use super::*;

        #[test]
        fn サニタイズされて引数パースエラーなく起動する() {
            // ヌルバイトはOSレベルでCLI引数に含められないため、
            // ユニットテスト（lib.rs）でカバーしている
            let mut cmd = Command::cargo_bin("love").unwrap();
            cmd.arg("--message").arg("Hello\x1b[31mWorld");
            cmd.timeout(Duration::from_millis(500));
            let output = cmd.output().expect("プロセスの実行に失敗");
            let stderr = String::from_utf8_lossy(&output.stderr);
            // clapの引数パースエラー（exit code 2）が出ていないことを確認
            assert_ne!(
                output.status.code(),
                Some(2),
                "引数パースエラー: {}",
                stderr
            );
        }
    }
}
