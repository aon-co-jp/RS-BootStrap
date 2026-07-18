//! RBootStrap — Bootstrap相当のCSSフレームワーク(グリッドシステム+
//! 基本コンポーネント)を、既存実装を一切流用せず一から開発する
//! プロジェクト(`RHTML5/RCSS3/RBootStrap/RTypeScript`構想の一部、
//! 2026-07-18新設)。セレクタ・宣言・カスケードの型は[`rcss3`]を
//! そのまま利用する(車輪の再発明をしない)。
//!
//! ## 現状(第一段)
//! グリッドシステム(`grid`モジュール、`container`/`container-fluid`/
//! `row`/`col-*`)と基本コンポーネント(`component`モジュール、
//! button/card/navbar)のクラス名パーサー+スタイル定義生成のみ。
//! 実際のflexbox折り返し計算・レスポンシブな`@media`出力・
//! フォーム/モーダル/ドロップダウン等の対話コンポーネントは未着手。

pub mod component;
pub mod grid;

pub use component::{
    button_declarations, card_declarations, navbar_declarations, parse_button_class, parse_card_class,
    parse_navbar_class, ButtonClass, ButtonSize, ButtonVariant, CardClass, NavbarClass,
};
pub use grid::{
    grid_declarations, parse_col_class, parse_grid_class, span_width_percent, Breakpoint, ColClass, ColSize,
    GridClass,
};

use rcss3::{Combinator, Declaration, Rule, Selector, SelectorSegment, SimplePart};

/// 単一のクラス名を、対応するCSS宣言列に変換する。グリッド系・
/// ボタン系・カード系・ナブバー系のいずれにもマッチしない場合は
/// `None`。
pub fn resolve_class(class: &str) -> Option<Vec<Declaration>> {
    if let Some(grid_class) = parse_grid_class(class) {
        return Some(grid_declarations(grid_class));
    }
    if let Some(button_class) = parse_button_class(class) {
        return Some(button_declarations(button_class));
    }
    if let Some(card_class) = parse_card_class(class) {
        return Some(card_declarations(card_class));
    }
    if let Some(navbar_class) = parse_navbar_class(class) {
        return Some(navbar_declarations(navbar_class));
    }
    None
}

/// クラス名1個を`rcss3::Rule`(セレクタ`.<class>` + 宣言列)に変換する。
/// `rcss3::compute_style`や`parse_stylesheet`の出力とそのまま
/// 組み合わせて使える。
pub fn class_rule(class: &str) -> Option<Rule> {
    let declarations = resolve_class(class)?;
    let selector: Selector =
        vec![SelectorSegment { combinator: Combinator::Descendant, compound: vec![SimplePart::Class(class.to_string())] }];
    Some(Rule { selectors: vec![selector], declarations })
}

/// 複数のクラス名から、マッチしたものだけをまとめて`Rule`列に変換する
/// (対応するクラスが1つも無ければ空の`Vec`)。
pub fn generate_stylesheet(classes: &[&str]) -> Vec<Rule> {
    classes.iter().filter_map(|c| class_rule(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_class_handles_grid_button_card_and_navbar() {
        assert!(resolve_class("col-6").is_some());
        assert!(resolve_class("btn-primary").is_some());
        assert!(resolve_class("card-body").is_some());
        assert!(resolve_class("navbar").is_some());
        assert_eq!(resolve_class("totally-unknown-class"), None);
    }

    #[test]
    fn class_rule_builds_a_class_selector_rule() {
        let rule = class_rule("btn").expect("btn should resolve");
        let expected_selector: Selector =
            vec![SelectorSegment { combinator: Combinator::Descendant, compound: vec![SimplePart::Class("btn".to_string())] }];
        assert_eq!(rule.selectors, vec![expected_selector]);
        assert!(rule.declarations.iter().any(|d| d.property == "cursor"));
    }

    #[test]
    fn generate_stylesheet_skips_unknown_classes() {
        let rules = generate_stylesheet(&["container", "not-a-class", "row"]);
        assert_eq!(rules.len(), 2);
    }
}
