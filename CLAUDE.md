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

## 現状(2026-07-20更新)

- `grid`モジュール: `container`/`container-fluid`/`row`/`col-*`(ブレーク
  ポイント`sm`/`md`/`lg`/`xl`対応)のクラス名パーサー(`parse_grid_class`/
  `parse_col_class`)とスタイル定義生成(`grid_declarations`)。
- `component`モジュール: button(variant/size)・card・navbarのクラス名
  パーサーとスタイル定義生成。
- `resolve_class`/`class_rule`/`generate_stylesheet`: 上記をまとめて
  クラス名→`rcss3::Rule`へ変換する統合API。`class_rule`は
  ブレークポイント付きクラス(`col-md-4`等)の場合、そのブレークポイントの
  `min-width`を`Rule.media`に実際に設定するようになった
  (2026-07-20、下記HANDOFF参照——従来は`ColClass.breakpoint`が
  パースされるだけで構築時に使われず、レスポンシブ`@media`出力は
  文字通り「未着手」だった)。
- 26テストgreen。

**未着手**: 実際のflexbox折り返し計算、フォーム/モーダル/ドロップダウン/
トースト等の対話コンポーネント、JavaScript連携(Bootstrap本家の
collapse/carousel等)。
**既知の設計上の限界(2026-07-20判明)**: `rcss3::compute_style`は
単一の`MediaContext`(既定screen・1920px)に対して1回だけ計算するSSR
向けの設計であり、実際の`@media (min-width: ...) { ... }`ブロックを
含むCSSテキストを出力するシリアライザが`rcss3`側に存在しない。
そのため、ブラウザ側でウィンドウ幅を変えた時にJSやページ再読み込み
無しで自動的にレイアウトが切り替わる、という本来のレスポンシブ挙動は
現状のアーキテクチャでは実現できない——`Rule.media`は正しく設定される
ようになったが、それを「本物のCSSファイル」として出力する経路が
無い、という次の課題。

## 関連プロジェクト

- [RFrontEnd](https://github.com/aon-co-jp/RFrontEnd) — 親リポジトリ
- [RCSS](https://github.com/aon-co-jp/RCSS) — 依存元(セレクタ/宣言/カスケード)
- [RHTML](https://github.com/aon-co-jp/RTHML) — 姉妹プロジェクト(DOM)
- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開発ルールの正本

## HANDOFF

- **2026-07-20 GitHubリポジトリ名を`RBootStrap`→`RBootstrap`にリネーム、
  ビルド破損の修正+レスポンシブ`@media`出力を実装**: 前回HANDOFFの
  記録に従い、同日実際に着手した。
  1. **リポジトリ名変更**: ユーザー指示により`gh repo rename`で
     GitHub上を`RBootStrap`→`RBootstrap`小文字化(実行前にローカル
     フルコピーをバックアップ済み)。ローカルフォルダ名
     (`F:\open-runo\RFrontEnd\RBootstrap`)・git remote URL・VPS側
     (`/root/RFrontEnd/RBootstrap`、従来は空の未クローンディレクトリ
     だったと判明——GitHub認証情報がVPSに無かったため、ローカルの
     完全なリポジトリをtar+scpで転送して復元)も合わせて更新。
  2. **ビルド破損の発見・修正**: `cargo test`を実行したところ、
     RCSS側が`Declaration`に`important`フィールド・`Rule`に`media`
     フィールドを追加していたのに、このリポジトリの`Declaration`/
     `Rule`リテラル(`component.rs`・`grid.rs`・`lib.rs`)が追従して
     おらず**ビルドが壊れていた**(誰にも気づかれず放置されていた
     実バグ)。両フィールドを追加して修正。
  3. **レスポンシブ`@media`出力の実装**: `class_rule()`が
     `col-md-4`等のブレークポイントを`parse_grid_class`で認識しては
     いたが、その情報を`Rule`構築時に一切使っておらず(パースする
     だけで捨てられていた)、CLAUDE.mdに「未着手」と明記されていた
     通り本当に未接続だった。ブレークポイントの`min-width`を
     `Rule.media`(`MediaQuery`)へ実際に設定するよう修正。
  - **検証**: `cargo test`22件→26件全green(新規4件: 非ブレークポイント
    クラスは`media`無し、`col-md-4`が`min-width:768px`を持つ、
    ブレークポイントごとに異なる`min-width`、非グリッドクラスは
    `media`を持たない、をそれぞれ確認)。`cargo build`警告0件。
  - **新たに判明した設計上の限界(正直な開示)**: 上記の「現状」節
    参照——`rcss3`側にCSSテキストシリアライザ(`@media`ブロックを
    含む実際の`.css`出力)が存在しないため、`Rule.media`を正しく
    設定しても、ブラウザ側での本当のレスポンシブ挙動(JS/再読み込み
    無しでの自動レイアウト切替)はまだ実現できない。
  - 次にすべきこと: (1) `rcss3`側にスタイルシート全体をCSSテキストへ
    シリアライズする関数(`@media`ブロック・複数`Rule`のグルーピングを
    含む)を追加する、(2) flexbox折り返し計算、(3) 対話コンポーネント
    (フォーム/モーダル/ドロップダウン/トースト)。

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
