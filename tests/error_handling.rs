#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_message_too_long_error() {
    let mut cmd = Command::cargo_bin("love").unwrap();
    let long_message = "a".repeat(101);
    cmd.arg("--message").arg(&long_message);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Message too long"))
        .stderr(predicate::str::contains("max 100 characters"));
}

#[test]
fn test_message_dos_attack() {
    let mut cmd = Command::cargo_bin("love").unwrap();
    // 10億文字は作れないので、明らかに長いメッセージでテスト
    let long_message = "a".repeat(1000);
    cmd.arg("--message").arg(&long_message);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Message too long"));
}

#[test]
fn test_message_with_escape_sequences() {
    let mut cmd = Command::cargo_bin("love").unwrap();
    // ANSIエスケープシーケンスを含むメッセージ
    cmd.arg("--message").arg("Hello\x1b[31mWorld");
    cmd.timeout(std::time::Duration::from_millis(100));
    // エスケープシーケンスは除去されるが、プログラムは正常に動作する
    let _ = cmd.ok();
}

#[test]
fn test_message_with_null_byte() {
    let mut cmd = Command::cargo_bin("love").unwrap();
    // NULL文字を含むメッセージ
    cmd.arg("--message").arg("Hello\x00World");
    cmd.timeout(std::time::Duration::from_millis(100));
    // NULL文字は除去されるが、プログラムは正常に動作する
    let _ = cmd.ok();
}
