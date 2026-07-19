//! グリッドシステム(`container`/`container-fluid`/`row`/`col-*`)の
//! クラス名パーサーとレイアウト計算ロジック。Bootstrapと互換の
//! クラス名体系(12カラム、`sm`/`md`/`lg`/`xl`/`xxl`ブレークポイント)
//! を認識するが、パース処理・幅計算ロジックは自作する。

use rcss3::Declaration;

/// レスポンシブブレークポイント。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl Breakpoint {
    /// このブレークポイントが有効になる最小ビューポート幅(px)。
    pub fn min_width_px(self) -> u32 {
        match self {
            Breakpoint::Sm => 576,
            Breakpoint::Md => 768,
            Breakpoint::Lg => 992,
            Breakpoint::Xl => 1200,
            Breakpoint::Xxl => 1400,
        }
    }

    fn parse(s: &str) -> Option<Breakpoint> {
        match s {
            "sm" => Some(Breakpoint::Sm),
            "md" => Some(Breakpoint::Md),
            "lg" => Some(Breakpoint::Lg),
            "xl" => Some(Breakpoint::Xl),
            "xxl" => Some(Breakpoint::Xxl),
            _ => None,
        }
    }
}

/// 列幅の指定。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColSize {
    /// コンテンツ幅に応じて自動(`col`, `col-md-auto`など)。
    Auto,
    /// 12カラム中の占有数(`col-6`など、1〜12)。
    Span(u8),
}

/// `col`/`col-6`/`col-md-4`/`col-auto`のようなクラス名の解析結果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColClass {
    pub breakpoint: Option<Breakpoint>,
    pub size: ColSize,
}

/// グリッド関連クラスの種類。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridClass {
    Container,
    ContainerFluid,
    Row,
    Col(ColClass),
}

const GRID_COLUMNS: u8 = 12;

/// `col`系クラス名をパースする。マッチしなければ`None`。
///
/// 対応パターン: `col`, `col-auto`, `col-<1..=12>`,
/// `col-<breakpoint>-auto`, `col-<breakpoint>-<1..=12>`
pub fn parse_col_class(class: &str) -> Option<ColClass> {
    let rest = class.strip_prefix("col")?;
    if rest.is_empty() {
        return Some(ColClass { breakpoint: None, size: ColSize::Auto });
    }
    let rest = rest.strip_prefix('-')?;
    let parts: Vec<&str> = rest.split('-').collect();
    match parts.as_slice() {
        [size] => parse_size(size).map(|size| ColClass { breakpoint: None, size }),
        [bp, size] => {
            let breakpoint = Breakpoint::parse(bp)?;
            parse_size(size).map(|size| ColClass { breakpoint: Some(breakpoint), size })
        }
        _ => None,
    }
}

fn parse_size(s: &str) -> Option<ColSize> {
    if s == "auto" {
        return Some(ColSize::Auto);
    }
    let n: u8 = s.parse().ok()?;
    if (1..=GRID_COLUMNS).contains(&n) {
        Some(ColSize::Span(n))
    } else {
        None
    }
}

/// グリッド関連クラス名(`container`/`container-fluid`/`row`/`col*`)を解析する。
pub fn parse_grid_class(class: &str) -> Option<GridClass> {
    match class {
        "container" => Some(GridClass::Container),
        "container-fluid" => Some(GridClass::ContainerFluid),
        "row" => Some(GridClass::Row),
        _ => parse_col_class(class).map(GridClass::Col),
    }
}

/// 12カラムグリッドにおける占有幅のパーセンテージを計算する。
pub fn span_width_percent(span: u8) -> f64 {
    100.0 * f64::from(span) / f64::from(GRID_COLUMNS)
}

/// グリッドクラスに対応するCSS宣言を生成する。
///
/// ブレークポイント自体のメディアクエリ生成(`@media`)は行わず、
/// ブレークポイント指定なしの宣言のみを返す(at-rule対応は`rcss3`側の
/// 今後の課題であり、それと足並みを揃えるスコープ)。
pub fn grid_declarations(class: GridClass) -> Vec<Declaration> {
    match class {
        GridClass::Container => vec![
            decl("width", "100%"),
            decl("margin-left", "auto"),
            decl("margin-right", "auto"),
            decl("padding-left", "0.75rem"),
            decl("padding-right", "0.75rem"),
        ],
        GridClass::ContainerFluid => {
            vec![decl("width", "100%"), decl("padding-left", "0.75rem"), decl("padding-right", "0.75rem")]
        }
        GridClass::Row => vec![
            decl("display", "flex"),
            decl("flex-wrap", "wrap"),
            decl("margin-left", "-0.75rem"),
            decl("margin-right", "-0.75rem"),
        ],
        GridClass::Col(col) => col_declarations(col),
    }
}

fn col_declarations(col: ColClass) -> Vec<Declaration> {
    match col.size {
        ColSize::Auto => vec![
            decl("flex", "0 0 auto"),
            decl("width", "auto"),
            decl("padding-left", "0.75rem"),
            decl("padding-right", "0.75rem"),
        ],
        ColSize::Span(n) => {
            let width = format!("{}%", trim_trailing_zeros(span_width_percent(n)));
            vec![
                decl("flex", "0 0 auto"),
                decl("width", &width),
                decl("padding-left", "0.75rem"),
                decl("padding-right", "0.75rem"),
            ]
        }
    }
}

/// 浮動小数点数を、末尾の余分なゼロを削った文字列に変換する
/// (`33.3333%`はそのまま、`50.0000%`は`50%`にする)。
fn trim_trailing_zeros(value: f64) -> String {
    let s = format!("{value:.4}");
    let s = s.trim_end_matches('0').trim_end_matches('.').to_string();
    if s.is_empty() {
        "0".to_string()
    } else {
        s
    }
}

fn decl(property: &str, value: &str) -> Declaration {
    Declaration { property: property.to_string(), value: value.to_string(), important: false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_col_as_auto() {
        assert_eq!(parse_col_class("col"), Some(ColClass { breakpoint: None, size: ColSize::Auto }));
    }

    #[test]
    fn parses_numbered_col() {
        assert_eq!(parse_col_class("col-6"), Some(ColClass { breakpoint: None, size: ColSize::Span(6) }));
    }

    #[test]
    fn parses_breakpoint_col() {
        assert_eq!(
            parse_col_class("col-md-4"),
            Some(ColClass { breakpoint: Some(Breakpoint::Md), size: ColSize::Span(4) })
        );
    }

    #[test]
    fn parses_breakpoint_auto_col() {
        assert_eq!(
            parse_col_class("col-lg-auto"),
            Some(ColClass { breakpoint: Some(Breakpoint::Lg), size: ColSize::Auto })
        );
    }

    #[test]
    fn rejects_out_of_range_span() {
        assert_eq!(parse_col_class("col-13"), None);
        assert_eq!(parse_col_class("col-0"), None);
    }

    #[test]
    fn rejects_unknown_breakpoint() {
        assert_eq!(parse_col_class("col-foo-4"), None);
    }

    #[test]
    fn rejects_unrelated_class() {
        assert_eq!(parse_col_class("button"), None);
    }

    #[test]
    fn span_width_percent_divides_evenly_for_common_spans() {
        assert!((span_width_percent(12) - 100.0).abs() < f64::EPSILON);
        assert!((span_width_percent(6) - 50.0).abs() < f64::EPSILON);
        assert!((span_width_percent(4) - 33.3333).abs() < 0.001);
    }

    #[test]
    fn parse_grid_class_recognizes_container_row_and_col() {
        assert_eq!(parse_grid_class("container"), Some(GridClass::Container));
        assert_eq!(parse_grid_class("container-fluid"), Some(GridClass::ContainerFluid));
        assert_eq!(parse_grid_class("row"), Some(GridClass::Row));
        assert!(matches!(parse_grid_class("col-6"), Some(GridClass::Col(_))));
        assert_eq!(parse_grid_class("not-a-grid-class"), None);
    }

    #[test]
    fn col_span_declarations_include_computed_width() {
        let decls = grid_declarations(GridClass::Col(ColClass { breakpoint: None, size: ColSize::Span(6) }));
        assert!(decls.iter().any(|d| d.property == "width" && d.value == "50%"));
    }

    #[test]
    fn container_declarations_center_with_auto_margins() {
        let decls = grid_declarations(GridClass::Container);
        assert!(decls.iter().any(|d| d.property == "margin-left" && d.value == "auto"));
    }
}
