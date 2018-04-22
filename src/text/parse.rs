use std::iter::Iterator;
use text::{Code, Token, Tokenizer, LEGACY_CHAR};
use text::chat::{Component, StringComponent, Wrapper, Chat};

fn strip_codes(codes: &[Code]) -> &[Code] {
    for (i, c) in codes.iter().enumerate() {
        if c.is_color() {
            return &codes[i..];
        }
    }
    codes
}

fn push_extra(c: &mut StringComponent, component: StringComponent) {
    let extra = &mut c.base.extra;

    if extra.is_none() {
        *extra = Some(Vec::new());
    }

    extra.as_mut().unwrap().push(Wrapper(Component::from(component)));
}

fn unwrap_string_component(c: &mut Component) -> &mut StringComponent {
    match *c {
        Component::String(ref mut string_component) => string_component,
        _ => unreachable!(),
    }
}

fn get_last_color(root: &mut StringComponent) -> &mut StringComponent {
    unwrap_string_component(&mut root.base.extra.as_mut().unwrap().last_mut().unwrap().0)
}

fn get_last_formatting(root: &mut StringComponent) -> &mut StringComponent {
    let mut formatting =
        unwrap_string_component(&mut get_last_color(root).base.extra.as_mut().unwrap()[0].0);

    loop {
        // move formatting so theres no multiple mut
        let tmp = formatting;
        // formatting is reassigned again in the match so it can be used later after loop.
        match tmp.base.extra {
            Some(ref mut extra) => formatting = unwrap_string_component(&mut extra[0].0),
            None => {
                formatting = tmp;
                break;
            }
        }
    }

    /* while let Some(ref mut extra) = formatting.base.extra {
        formatting = unwrap_string_component(&mut extra[0].0);
    } */

    formatting
}

/// Parses legacy minecraft chat into json chat format. Not the most optimized but works well.
/// You need to use the LEGACY_CHAR for formatting. Code implements display which uses LEGACY_CHAR so use format! here.
pub fn parse_legacy(s: &str) -> Component {
    parse_legacy_ex(s, LEGACY_CHAR)
}

/// parse_legacy, but allows u to change control character from LEGACY_CHAR to something else. Usually '&'
pub fn parse_legacy_ex(s: &str, control_char: char) -> Component {
    /*
    ha &k&o aa &4 hi &l hello &o&k world &a hola
    
    This is parsed as follows. Root is a collection of color (at least one). 
    Each color is has exactly one formatting. 
    Each formatting has an optional exactly one formatting.

    Eg: 

    root
    |--color
    |  |--formatting "ha "
    |     |--formatting &k&o " aa "
    |--color &4
    |  |--formatting " hi "
    |     |--formatting &l " hello "
    |        |--formatting &o&k " world "
    |--color &a
    |  |--formatting "hola"
    */

    let mut tokenizer = Tokenizer::new(s, control_char);

    let mut root = StringComponent::default();
    push_extra(&mut root, StringComponent::default()); // color
    push_extra(get_last_color(&mut root), StringComponent::default()); //formatting

    while let Some(token) = Iterator::next(&mut tokenizer) {
        match token {
            Token::String(x) => {
                let formatting = get_last_formatting(&mut root);
                formatting.text.push_str(&x);
            }
            Token::Codes(x) => {
                let codes = strip_codes(&x);
                if codes[0].is_formatting() {
                    {
                        let mut formatting = get_last_formatting(&mut root);
                        push_extra(formatting, StringComponent::default());
                    }
                    let formatting = get_last_formatting(&mut root);
                    for &c in codes {
                        formatting.base.set_formatting_style(c);
                    }
                } else {
                    push_extra(&mut root, StringComponent::default()); //color
                    {
                        let color = get_last_color(&mut root);
                        push_extra(color, StringComponent::default()); //formatting
                        color.base.color = Some(codes[0].to_color());
                    }
                    let formatting = get_last_formatting(&mut root);
                    for &c in &codes[1..] {
                        formatting.base.set_formatting_style(c);
                    }
                }
            }
        }
    }

    Component::from(root)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let s = "&6&l&kii&4&lWigit&6&l&kii";
        let component = parse_legacy_ex(s, '&');
        let res = ::serde_json::to_string(&Chat::from(component)).unwrap();
        assert_eq!(res, r#"{"extra":[{"extra":[{"text":""}],"text":""},{"color":"gold","extra":[{"bold":true,"obfuscated":true,"text":"ii"}],"text":""},{"color":"dark_red","extra":[{"bold":true,"text":"Wigit"}],"text":""},{"color":"gold","extra":[{"bold":true,"obfuscated":true,"text":"ii"}],"text":""}],"text":""}"#);
    }
}