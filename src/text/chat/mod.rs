use std::default::Default;

mod color;
pub use self::color::*;

mod events;
pub use self::events::*;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")] bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")] insertion: Option<String>,
    #[serde(rename = "clickEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    click_event: Option<ClickEvent>,
    #[serde(rename = "hoverEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_event: Option<HoverEvent>,
    #[serde(skip_serializing_if = "Option::is_none")] extra: Option<Vec<Component>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringComponent {
    #[serde(flatten)]
    pub base: Base,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationComponent {
    #[serde(flatten)]
    pub base: Base,
    pub translate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub with: Option<Vec<Component>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub name: String,
    pub objective: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreComponent {
    #[serde(flatten)]
    pub base: Base,
    pub score: Score,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectorComponent {
    #[serde(flatten)]
    pub base: Base,
    pub selector: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Component {
    String(StringComponent),
    Translation(TranslationComponent),
    Score(ScoreComponent),
    Selector(SelectorComponent),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_enum_ser() {
        let e = Color::Black;
        let st = serde_json::to_string(&e).unwrap();
        assert_eq!(st, "\"black\"");
    }

    #[test]
    fn test_enum_de() {
        let st = "\"black\"";
        let color: Color = serde_json::from_str(st).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_enum_de_wrong() {
        let st = "garbage";
        let color: Color = serde_json::from_str(st).unwrap();
    }

    #[test]
    fn test_ser() {
        let com = Component::String(StringComponent{
            base: Base {bold: Some(true), ..Default::default()},
            text: "Hello".to_string(),
        });
        let st = serde_json::to_string(&com).unwrap();
        assert_eq!(st, "{\"bold\":true,\"text\":\"Hello\"}");
    }

    #[test]
    fn test_de() {
        let com: Component = serde_json::from_str("{\"bold\":true,\"text\":\"Hello\"}").unwrap();
        match com {
            Component::String(s) => {
                assert_eq!(s.base.bold, Some(true));
                assert_eq!(s.text, "Hello");
            },
            _ => panic!(),
        }
    }
}
