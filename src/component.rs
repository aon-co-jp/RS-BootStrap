//! ボタン/カード/ナブバーなど基本コンポーネントのクラス名から
//! スタイル定義への変換。Bootstrapのクラス名体系(`btn-primary`・
//! `card-body`・`navbar-brand`等)とは互換だが、実際に割り当てる
//! 色・寸法の数値は本クレートで新たに定めた値であり、既存CSS
//! ソースからの丸写しはしない。

use rcss3::Declaration;

/// ボタンの配色バリアント。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl ButtonVariant {
    fn parse(s: &str) -> Option<ButtonVariant> {
        match s {
            "primary" => Some(ButtonVariant::Primary),
            "secondary" => Some(ButtonVariant::Secondary),
            "success" => Some(ButtonVariant::Success),
            "danger" => Some(ButtonVariant::Danger),
            "warning" => Some(ButtonVariant::Warning),
            "info" => Some(ButtonVariant::Info),
            "light" => Some(ButtonVariant::Light),
            "dark" => Some(ButtonVariant::Dark),
            _ => None,
        }
    }

    /// (背景色, 前景色)。
    fn colors(self) -> (&'static str, &'static str) {
        match self {
            ButtonVariant::Primary => ("#3363ff", "#ffffff"),
            ButtonVariant::Secondary => ("#6c7480", "#ffffff"),
            ButtonVariant::Success => ("#2f9e44", "#ffffff"),
            ButtonVariant::Danger => ("#d6336c", "#ffffff"),
            ButtonVariant::Warning => ("#f2b705", "#1a1a1a"),
            ButtonVariant::Info => ("#22b8cf", "#ffffff"),
            ButtonVariant::Light => ("#f4f4f5", "#1a1a1a"),
            ButtonVariant::Dark => ("#22252a", "#ffffff"),
        }
    }
}

/// ボタンのサイズ修飾子。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Large,
}

/// `btn`系クラスの解析結果。1クラス名=1バリエーション
/// (`btn`, `btn-primary`, `btn-outline-primary`, `btn-sm`, `btn-lg`)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonClass {
    Base,
    Variant(ButtonVariant),
    OutlineVariant(ButtonVariant),
    Size(ButtonSize),
}

/// `btn`系クラス名をパースする。マッチしなければ`None`。
pub fn parse_button_class(class: &str) -> Option<ButtonClass> {
    if class == "btn" {
        return Some(ButtonClass::Base);
    }
    let rest = class.strip_prefix("btn-")?;
    match rest {
        "sm" => Some(ButtonClass::Size(ButtonSize::Small)),
        "lg" => Some(ButtonClass::Size(ButtonSize::Large)),
        _ => {
            if let Some(variant) = rest.strip_prefix("outline-") {
                ButtonVariant::parse(variant).map(ButtonClass::OutlineVariant)
            } else {
                ButtonVariant::parse(rest).map(ButtonClass::Variant)
            }
        }
    }
}

/// ボタンクラスに対応するCSS宣言を生成する。
pub fn button_declarations(class: ButtonClass) -> Vec<Declaration> {
    match class {
        ButtonClass::Base => vec![
            decl("display", "inline-block"),
            decl("padding", "0.375rem 0.75rem"),
            decl("border-radius", "0.375rem"),
            decl("border", "1px solid transparent"),
            decl("font-size", "1rem"),
            decl("text-align", "center"),
            decl("cursor", "pointer"),
        ],
        ButtonClass::Variant(v) => {
            let (bg, fg) = v.colors();
            vec![decl("background-color", bg), decl("border-color", bg), decl("color", fg)]
        }
        ButtonClass::OutlineVariant(v) => {
            let (bg, _fg) = v.colors();
            vec![decl("background-color", "transparent"), decl("border-color", bg), decl("color", bg)]
        }
        ButtonClass::Size(ButtonSize::Small) => {
            vec![decl("padding", "0.25rem 0.5rem"), decl("font-size", "0.875rem")]
        }
        ButtonClass::Size(ButtonSize::Large) => {
            vec![decl("padding", "0.5rem 1rem"), decl("font-size", "1.25rem")]
        }
    }
}

/// カード関連クラス。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardClass {
    Card,
    CardBody,
    CardTitle,
    CardText,
    CardHeader,
    CardFooter,
}

/// `card`系クラス名をパースする。マッチしなければ`None`。
pub fn parse_card_class(class: &str) -> Option<CardClass> {
    match class {
        "card" => Some(CardClass::Card),
        "card-body" => Some(CardClass::CardBody),
        "card-title" => Some(CardClass::CardTitle),
        "card-text" => Some(CardClass::CardText),
        "card-header" => Some(CardClass::CardHeader),
        "card-footer" => Some(CardClass::CardFooter),
        _ => None,
    }
}

/// カードクラスに対応するCSS宣言を生成する。
pub fn card_declarations(class: CardClass) -> Vec<Declaration> {
    match class {
        CardClass::Card => vec![
            decl("display", "flex"),
            decl("flex-direction", "column"),
            decl("border", "1px solid #d8dbe0"),
            decl("border-radius", "0.5rem"),
            decl("background-color", "#ffffff"),
        ],
        CardClass::CardBody => vec![decl("padding", "1rem")],
        CardClass::CardTitle => {
            vec![decl("font-size", "1.25rem"), decl("font-weight", "600"), decl("margin-bottom", "0.5rem")]
        }
        CardClass::CardText => vec![decl("margin-bottom", "1rem")],
        CardClass::CardHeader => vec![decl("padding", "0.75rem 1rem"), decl("border-bottom", "1px solid #d8dbe0")],
        CardClass::CardFooter => vec![decl("padding", "0.75rem 1rem"), decl("border-top", "1px solid #d8dbe0")],
    }
}

/// ナブバー関連クラス。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavbarClass {
    Navbar,
    NavbarBrand,
    NavbarNav,
    NavItem,
    NavLink,
}

/// `navbar`/`nav`系クラス名をパースする。マッチしなければ`None`。
pub fn parse_navbar_class(class: &str) -> Option<NavbarClass> {
    match class {
        "navbar" => Some(NavbarClass::Navbar),
        "navbar-brand" => Some(NavbarClass::NavbarBrand),
        "navbar-nav" => Some(NavbarClass::NavbarNav),
        "nav-item" => Some(NavbarClass::NavItem),
        "nav-link" => Some(NavbarClass::NavLink),
        _ => None,
    }
}

/// ナブバークラスに対応するCSS宣言を生成する。
pub fn navbar_declarations(class: NavbarClass) -> Vec<Declaration> {
    match class {
        NavbarClass::Navbar => {
            vec![decl("display", "flex"), decl("align-items", "center"), decl("padding", "0.5rem 1rem")]
        }
        NavbarClass::NavbarBrand => {
            vec![decl("font-size", "1.25rem"), decl("font-weight", "700"), decl("margin-right", "1rem")]
        }
        NavbarClass::NavbarNav => {
            vec![decl("display", "flex"), decl("flex-direction", "row"), decl("list-style", "none")]
        }
        NavbarClass::NavItem => vec![decl("margin-left", "0.5rem")],
        NavbarClass::NavLink => vec![decl("padding", "0.5rem 0.75rem"), decl("text-decoration", "none")],
    }
}

fn decl(property: &str, value: &str) -> Declaration {
    Declaration { property: property.to_string(), value: value.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_base_and_variant_button_classes() {
        assert_eq!(parse_button_class("btn"), Some(ButtonClass::Base));
        assert_eq!(parse_button_class("btn-primary"), Some(ButtonClass::Variant(ButtonVariant::Primary)));
    }

    #[test]
    fn parses_outline_button_classes() {
        assert_eq!(parse_button_class("btn-outline-danger"), Some(ButtonClass::OutlineVariant(ButtonVariant::Danger)));
    }

    #[test]
    fn parses_button_size_classes() {
        assert_eq!(parse_button_class("btn-sm"), Some(ButtonClass::Size(ButtonSize::Small)));
        assert_eq!(parse_button_class("btn-lg"), Some(ButtonClass::Size(ButtonSize::Large)));
    }

    #[test]
    fn rejects_unknown_button_variant() {
        assert_eq!(parse_button_class("btn-notacolor"), None);
    }

    #[test]
    fn outline_button_uses_transparent_background() {
        let decls = button_declarations(ButtonClass::OutlineVariant(ButtonVariant::Primary));
        assert!(decls.iter().any(|d| d.property == "background-color" && d.value == "transparent"));
    }

    #[test]
    fn parses_card_classes() {
        assert_eq!(parse_card_class("card"), Some(CardClass::Card));
        assert_eq!(parse_card_class("card-body"), Some(CardClass::CardBody));
        assert_eq!(parse_card_class("not-a-card-class"), None);
    }

    #[test]
    fn parses_navbar_classes() {
        assert_eq!(parse_navbar_class("navbar"), Some(NavbarClass::Navbar));
        assert_eq!(parse_navbar_class("nav-link"), Some(NavbarClass::NavLink));
        assert_eq!(parse_navbar_class("not-navbar"), None);
    }

    #[test]
    fn navbar_is_a_flex_container() {
        let decls = navbar_declarations(NavbarClass::Navbar);
        assert!(decls.iter().any(|d| d.property == "display" && d.value == "flex"));
    }
}
