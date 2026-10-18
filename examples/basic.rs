use rs_bootstrap::{class_rule, generate_stylesheet};

fn main() {
    // 1クラス名 → CSSルール / one class name -> one CSS rule
    let rule = class_rule("col-md-4").expect("known class");
    println!("col-md-4: {} declaration(s), media = {:?}", rule.declarations.len(), rule.media);

    // 複数クラス名 → 該当するものだけのルール列 / many class names -> rules for the ones that match
    let rules = generate_stylesheet(&["container", "row", "col-6", "not-a-bootstrap-class"]);
    println!("generated {} rule(s) (unknown classes are skipped)", rules.len());
}
