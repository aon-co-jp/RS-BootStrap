# PORTING.md — RBootStrap を他プロジェクトへお引越しする際のガイド

## クラス名→CSS宣言の変換パターン

`resolve_class(class: &str) -> Option<Vec<Declaration>>`(`src/lib.rs`)
が、グリッド系・コンポーネント系のいずれのクラス名パーサーにも
マッチしない場合に`None`を返す設計は、新しいクラス体系(将来の
フォーム/モーダル対応等)を追加する際にそのまま踏襲できる
(`if let Some(...) = parse_xxx_class(class) { return ...; }`を
追記していくだけでよい)。

## RCSSとの依存パターン

`Cargo.toml`で`rcss3 = { path = "../RCSS" }`のようにpath依存し、
`Declaration`/`Rule`/`Selector`/`SimplePart`をそのまま再利用する。
セレクタ・宣言の型を自前で持たず、既存のCSS処理系にただ乗りする
設計は、他のCSSフレームワーク実装(Tailwind相当等)を作る際にも
応用できる。

## 注意事項

- 実際のレイアウト計算(flexbox折り返し等)は本リポジトリでは行わず、
  あくまで「クラス名→静的CSS宣言」の変換に留める設計。実レイアウト
  エンジンが必要な場合はRHTML/RCSS側のレンダリングパイプラインに
  委ねること。
