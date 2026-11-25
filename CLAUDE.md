# 開発方針＆開発環境ルール(RS-BootStrap)

作業ドライブは`F:\runo`。この節は[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の
`CLAUDE.md`を正本とし、各プロジェクトへコピーして同期する方針に準じる。

## リポジトリ改称(2026-09-13)

`RBootstrap`→`RS-BootStrap`へGitHub上でrename済み(crate名も
`rbootstrap`→`rs-bootstrap`)。`aruaru.pro`(コンコナラ型スキル
マーケットプレイス+求人サイト統合の新規プロジェクト)向けの
フロント基盤整備に合わせた、`RFrontEnd`傘下プロジェクトのネーミング
統一の一環(`RJSON`→`RS-JSON`統合・`RTypeScript`→`RS-TypeScript`・
`RS-JavaScript`新設)。以下の記述内の`RBootStrap`/`RBootstrap`表記は
改称前の履歴として残す(過去に`RBootStrap`→`RBootstrap`の大文字/小文字
リネームが既にあったため、今回で名称変更は2回目)。

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

- **2026-10-18 v0.1.0公開**: rcss3をpath依存からgit依存(aon-co-jp/RS-CSS)へ変更し単体でビルド可能に、examples/basic.rs・LICENSE(Apache-2.0 OR MIT)追加、全履歴の機密検査後に公開。フォーム/モーダル/ドロップダウン等の対話コンポーネント、実flexbox折り返し計算は未着手のまま(ライブラリのためバイナリRelease無し)。

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.

## API 方針の補足: 用語の整理と使い分け (2026-11-04)

### 日本語
**1つに統一するものではなく、別の層の話です。** 「REST の代わり」になるのは Connect・gRPC・GraphQL だけで、WASM は画面を作る技術です。

| 用語 | 何か | REST の代わりか |
|---|---|---|
| REST | URL と HTTP メソッドで操作を表す API の作り方 | 全廃する側 |
| **Connect** | `POST /サービス名/メソッド名` に JSON を送る形式。`curl` でもブラウザの `fetch` でも呼べる | はい(**採用**) |
| **gRPC** | 同じ「サービス/メソッド」の考え方を、バイナリ(protobuf)と HTTP/2 で行う形式。速く、ストリーミングも得意 | はい(**サービス同士に採用**) |
| gRPC-web | gRPC のブラウザ向けの変形版 | はい(必要なら) |
| GraphQL | 欲しい項目を問い合わせる形式 | 一部だけ(**問い合わせ(読み取り)専用**) |
| WASM | Rust をブラウザの中で動かす仕組み | いいえ(画面を作る技術) |

- **ブラウザの画面や `curl` から呼ぶ API は Connect**(POST、JSON を RS-JSON で読み書き)。ブラウザの WASM は素の gRPC を話せず、Connect なら `fetch` のままで、`prost` を足さずに済む(画面のサイズも増えない)。
- **サービス同士の通信(サーバー間、重い処理、ストリーミング)は gRPC。** 多くは Connect で足りるので、ストリーミングが必要になった所だけ gRPC に上げる。
- **GraphQL は、まとめて問い合わせたいときの読み取り専用。** 書き込み(操作)には使わない。
- **WASM は画面を作る技術として、そのまま使う。** 画面から呼ぶ API が Connect になっただけ。
- 外部の規格が形を決めるもの(ACME、OAuth のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)は、規格どおりの HTTP のまま。
- **迷ったら Connect。**

### English
**These are not alternatives to unify; they sit at different layers.** Only Connect, gRPC and GraphQL can replace REST; WASM is a way to build the UI.

| Term | What it is | Replaces REST? |
|---|---|---|
| REST | API style that expresses operations through URLs and HTTP methods | The thing being abolished |
| **Connect** | `POST /Service/Method` with a JSON body; callable from `curl` and from a browser `fetch` | Yes (**adopted**) |
| **gRPC** | The same service/method idea over binary (protobuf) and HTTP/2; fast, good at streaming | Yes (**for service-to-service**) |
| gRPC-web | A browser-friendly variant of gRPC | Yes (if needed) |
| GraphQL | A query language: ask for exactly the fields you want | Partly (**read-only queries**) |
| WASM | Runs Rust inside the browser | No (a UI technology) |

- **APIs called from browser screens or `curl` use Connect** (POST, JSON read and written with RS-JSON). Browser WASM cannot speak plain gRPC; with Connect it stays a `fetch`, and `prost` is not needed (the binary does not grow).
- **Service-to-service calls (server to server, heavy work, streaming) use gRPC.** Connect is enough in most places; move a call to gRPC only where streaming is actually needed.
- **GraphQL is read-only**, for queries that gather several pieces of data. It is not used for writes (operations).
- **WASM stays as the way to build the UI.** The API it calls simply became Connect.
- Interfaces whose shape an outside standard dictates (ACME, OAuth callbacks, third-party webhooks, health checks, static files, OTLP) stay plain HTTP.
- **When in doubt, use Connect.**

### Deutsch (KI-Übersetzung, sprachlich überarbeitet; noch nicht von Muttersprachlern geprüft)
**Es geht nicht darum, eine einzige Technik auszuwählen – die Begriffe liegen auf unterschiedlichen Ebenen.** Als Ersatz für REST kommen nur Connect, gRPC und GraphQL in Frage; WASM dagegen ist eine Technik, mit der die Benutzeroberfläche gebaut wird.

| Begriff | Was ist das? | Ersatz für REST? |
|---|---|---|
| REST | Ein API-Stil, bei dem Operationen über URLs und HTTP-Methoden ausgedrückt werden | wird abgeschafft |
| **Connect** | `POST /Dienst/Methode` mit JSON im Body; aufrufbar mit `curl` ebenso wie mit `fetch` im Browser | ja (**gewählt**) |
| **gRPC** | Dasselbe Prinzip aus Dienst und Methode, aber über binäres protobuf und HTTP/2; schnell und gut für Streaming geeignet | ja (**zwischen Diensten**) |
| gRPC-web | Eine für den Browser angepasste Variante von gRPC | ja (bei Bedarf) |
| GraphQL | Abfragesprache, mit der man genau die benötigten Felder anfordert | teilweise (**nur für lesende Abfragen**) |
| WASM | Führt Rust-Code direkt im Browser aus | nein (Technik für die Oberfläche) |

- **APIs, die von Browseroberflächen oder per `curl` aufgerufen werden, verwenden Connect** (POST; JSON wird mit RS-JSON gelesen und geschrieben). WASM im Browser kann kein reines gRPC sprechen. Mit Connect bleibt es bei einem gewöhnlichen `fetch`, und `prost` ist nicht nötig – die WASM-Datei wird dadurch nicht größer.
- **Für die Kommunikation zwischen Diensten (Server zu Server, rechenintensive Aufgaben, Streaming) wird gRPC eingesetzt.** In den meisten Fällen reicht Connect aus; auf gRPC wird nur dort umgestellt, wo Streaming tatsächlich gebraucht wird.
- **GraphQL dient ausschließlich lesenden Abfragen**, wenn mehrere Informationen gebündelt abgefragt werden sollen. Für Schreibzugriffe (Operationen) wird es nicht verwendet.
- **WASM bleibt die Technik für die Oberfläche.** Geändert hat sich lediglich, dass die aufgerufene API nun auf Connect beruht.
- Schnittstellen, deren Form durch einen externen Standard festgelegt ist (ACME, OAuth-Callbacks, Webhooks von Drittanbietern, Health-Checks, statische Dateien, OTLP), bleiben gewöhnliches HTTP.
- **Im Zweifel: Connect.**

### Русский (перевод выполнен с помощью ИИ и литературно отредактирован; носителями языка пока не проверен)
**Это не варианты, из которых нужно выбрать один, а технологии разных уровней.** Заменить REST могут только Connect, gRPC и GraphQL; WASM же — это способ создания пользовательского интерфейса.

| Термин | Что это | Заменяет REST? |
|---|---|---|
| REST | Стиль построения API, при котором операции выражаются через URL и HTTP-методы | отменяется |
| **Connect** | `POST /Сервис/Метод` с телом в формате JSON; вызывается как из `curl`, так и из браузерного `fetch` | да (**выбран**) |
| **gRPC** | Тот же принцип «сервис — метод», но поверх двоичного формата protobuf и HTTP/2; работает быстро и хорошо подходит для потоковой передачи | да (**для связи между сервисами**) |
| gRPC-web | Адаптированный для браузера вариант gRPC | да (при необходимости) |
| GraphQL | Язык запросов: клиент запрашивает ровно те поля, которые ему нужны | частично (**только запросы на чтение**) |
| WASM | Позволяет выполнять код на Rust прямо в браузере | нет (технология для интерфейса) |

- **API, которые вызываются из браузерных интерфейсов или через `curl`, строятся на Connect** (POST; JSON читается и записывается через RS-JSON). Браузерный WASM не умеет работать с «чистым» gRPC. С Connect достаточно обычного `fetch`, а `prost` не требуется — размер WASM-файла не увеличивается.
- **Для обмена между сервисами (сервер — сервер, ресурсоёмкие задачи, потоковая передача) применяется gRPC.** В большинстве случаев достаточно Connect; на gRPC переходят только там, где потоковая передача действительно необходима.
- **GraphQL используется только для чтения**, когда нужно получить сразу несколько связанных данных. Для записи (операций) он не применяется.
- **WASM по-прежнему остаётся технологией для интерфейса.** Изменилось лишь то, что вызываемый им API теперь построен на Connect.
- Интерфейсы, форма которых задана внешним стандартом (ACME, обратные вызовы OAuth, вебхуки сторонних сервисов, проверки работоспособности, статические файлы, OTLP), остаются обычным HTTP.
- **Если есть сомнения — выбирайте Connect.**

## Versionless API の方針 / Versionless API policy (2026-11-25)
**日本語**: このリポジトリのAPIは **Versionless**(URL・パッケージ名に `v1` 等を付けず、互換性を壊さず足すだけで進化させる)。欄は足すだけ・足す欄は省略可能・知らない欄は無視・名前変更は新名を足して旧を非推奨に(期限は既定で6か月以上、使用0を確認してから削除)・enumは未知の値を許容・安全上の緊急修正は例外。既存の `.v1` は呼び出し側を壊さないよう当面残し、版なしの別名を追加して段階的に畳む。詳細・出典は [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md) 末尾。適用はこれから順に行う。
**English**: This repository's APIs are **Versionless**: no `v1` in URLs or package names; evolve by adding only, without breaking compatibility. Only add fields, new fields are optional, ignore unknown fields, rename by adding the new name and deprecating the old (default deadline 6 months or more, remove only after confirming zero use), tolerate unknown enum values, emergency security fixes are the exception. Existing `.v1` names stay for now so callers do not break; a versionless alias is added and `.v1` is retired in stages. Details and sources: the end of [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md). Rollout is step by step.
