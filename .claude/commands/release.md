# release — バージョンリリース

引数 `$ARGUMENTS` からバージョン番号を受け取り、リリースを実行する。

## 手順

1. 引数の検証
   - `$ARGUMENTS` が semver 形式（例: `1.3.0`）であることを確認
   - 空の場合はエラーメッセージを表示して終了

2. 事前チェック
   - `main` ブランチにいることを確認（違う場合はエラー）
   - ワーキングツリーがクリーンであることを確認
   - `git pull` で最新状態に更新
   - `gh run list -w CI -b main -L 1 --json conclusion --jq '.[0].conclusion'` で main の最新 CI が `success` であることを確認（失敗またはin_progressの場合はエラーで終了）
   - `cargo fmt --check` でフォーマット確認
   - `cargo clippy -- -D warnings` でlint確認
   - `cargo test` でテスト実行
   - 全て通らなければエラーで終了

3. バージョンバンプ
   - `Cargo.toml` の `version` フィールドを `$ARGUMENTS` に更新
   - `cargo check` を実行して `Cargo.lock` を更新

4. コミットとタグ
   - `Cargo.toml` と `Cargo.lock` をステージング
   - コミットメッセージ: `chore: bump version to $ARGUMENTS`
   - タグ `v$ARGUMENTS` を作成

5. プッシュ
   - コミットとタグを `git push origin main --tags` でプッシュ

6. 完了報告
   - リリースワークフローの状態を `gh run list -w Release -L 1` で表示
   - ユーザーに完了を報告し、GitHub Actions のURLを案内
