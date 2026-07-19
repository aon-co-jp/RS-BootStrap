# 開発方針＆開発環境ルール(RBootStrap)

作業ドライブは`F:\open-runo`。この節は[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の
`CLAUDE.md`を正本とし、各プロジェクトへコピーして同期する方針に準じる。

## このリポジトリの役割

[RFrontEnd](https://github.com/aon-co-jp/RFrontEnd)傘下、Bootstrap相当の
CSSフレームワークを既存実装を一切流用せず一から開発するプロジェクト
(2026-07-18新設)。開発順序: RHTML/RCSS(土台)→ **RBootStrap** →
RTypeScript(RReactは別プロジェクトで並行開発)。

## 技術スタック

Rust。セレクタ・宣言・カスケードの型は[RCSS](https://github.com/aon-co-jp/RCSS)
(`rcss3`クレート)をpath依存でそのまま利用し、車輪の再発明をしない。
`unsafe_code = "deny"`。

## 現状(v0.1.0)

- `grid`モジュール: `container`/`container-fluid`/`row`/`col-*`(ブレーク
  ポイント`sm`/`md`/`lg`/`xl`対応)のクラス名パーサー(`parse_grid_class`/
  `parse_col_class`)とスタイル定義生成(`grid_declarations`)。
- `component`モジュール: button(variant/size)・card・navbarのクラス名
  パーサーとスタイル定義生成。
- `resolve_class`/`class_rule`/`generate_stylesheet`: 上記をまとめて
  クラス名→`rcss3::Rule`へ変換する統合API。
- 22テストgreen。

**未着手**: 実際のflexbox折り返し計算、レスポンシブな`@media`出力、
フォーム/モーダル/ドロップダウン/トースト等の対話コンポーネント、
JavaScript連携(Bootstrap本家のcollapse/carousel等)。

## 関連プロジェクト

- [RFrontEnd](https://github.com/aon-co-jp/RFrontEnd) — 親リポジトリ
- [RCSS](https://github.com/aon-co-jp/RCSS) — 依存元(セレクタ/宣言/カスケード)
- [RHTML](https://github.com/aon-co-jp/RTHML) — 姉妹プロジェクト(DOM)
- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開発ルールの正本

## HANDOFF

- **2026-07-19 audiocafe-tokyo-rustユーザーからの完成度向上要望を記録
  (次回セッションの着手事項)**: `audiocafe-tokyo-rust`のユーザーから
  「未着手や未完成の技術があれば、それぞれのリポジトリもTESTしながら
  完成させていって、実用性と完成度を高めていって下さい」という要望が
  あった。各リポジトリを別セッションで順に実施する方針となったため、
  本セッションでは実装は行わず記録のみ。**RFrontEnd傘下の中では
  コミット数・実装範囲ともに最も進捗が浅いリポジトリ**(コミット2件、
  「未着手」節にある通りflexbox折り返し計算・レスポンシブ`@media`
  出力・対話コンポーネントがいずれも手つかず)——エコシステム全体の
  完成度を上げる作業に着手する場合、優先度を高くしてよい候補。
  次回セッションでは「未着手」節の項目(まずflexbox折り返し計算・
  `@media`出力)へ確認不要でそのまま着手すること。

- **2026-07-18 新規作成**: RFrontEndの開発順序(RHTML/RCSS→RBootStrap→
  RTypeScript)に沿って新設。グリッド+基本コンポーネントのクラス名
  パーサーを実装、22テストgreen。**その後、独立したGitHubリポジトリとして
  初期化・push忘れが発覚**(RFrontEnd配下の未追跡ディレクトリのまま
  放置されていた)——`git init`・ドキュメント整備の上、本コミットで
  [aon-co-jp/RBootStrap](https://github.com/aon-co-jp/RBootStrap)へ
  初回push。次回以降: flexbox計算・レスポンシブ`@media`出力の実装。
