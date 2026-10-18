# RBootStrap

Bootstrap相当のCSSフレームワーク(グリッドシステム+基本コンポーネント)を、
既存実装を一切流用せず一から開発するプロジェクト。[RFrontEnd](https://github.com/aon-co-jp/RFrontEnd)
(RHTML/RCSS/RTypeScript/RJSON/RReact)の一部で、開発順序上は
RHTML/RCSS(土台)の次段階にあたる。

## 現状(v0.1.0、第一段)

- **グリッドシステム**(`grid`モジュール): `container`/`container-fluid`/
  `row`/`col-*`のクラス名パーサーとスタイル定義生成。
- **基本コンポーネント**(`component`モジュール): button/card/navbarの
  クラス名パーサーとスタイル定義生成。
- セレクタ・宣言・カスケードの型は[RCSS](https://github.com/aon-co-jp/RCSS)
  をそのまま利用し、車輪の再発明をしない。

**未着手**: 実際のflexbox折り返し計算、レスポンシブな`@media`出力、
フォーム/モーダル/ドロップダウン等の対話コンポーネント。

## 技術スタック

Rust。`rcss3`(RCSS)への依存以外、外部クレートなし。

## 関連プロジェクト

- [RFrontEnd](https://github.com/aon-co-jp/RFrontEnd) — 親リポジトリ(全体構成)
- [RCSS](https://github.com/aon-co-jp/RCSS) — セレクタ/宣言/カスケードの型を提供
- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開発ルールの正本

## �g���� / Usage

`cargo run --example basic`(`examples/basic.rs`)���Q�ƁB�ˑ� `rcss3` ��GitHub�� [RS-CSS](https://github.com/aon-co-jp/RS-CSS) ����擾���܂��B / See `examples/basic.rs` (`cargo run --example basic`). The `rcss3` dependency is fetched from [RS-CSS](https://github.com/aon-co-jp/RS-CSS).

## ���C�Z���X / License

Apache-2.0 �܂��� MIT(�ǂ��炩�I��)�B / Apache-2.0 OR MIT, at your option. See `LICENSE-APACHE` / `LICENSE-MIT`.

