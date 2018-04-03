use std::fmt;
use std::convert::From;
use serde::de::{Deserialize, Deserializer, SeqAccess, MapAccess, Visitor, self};
use std::io;
use serde_json;
use proto;

use super::Code;

mod color;
pub use self::color::*;

mod events;
pub use self::events::*;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")] pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")] pub insertion: Option<String>,
    #[serde(rename = "clickEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    #[serde(rename = "hoverEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
    #[serde(skip_serializing_if = "Option::is_none")] pub extra: Option<Vec<ComponentWrapper>>,
}

impl Base {
    pub fn set_formatting_style(&mut self, c: Code) {
        match c {
            Code::Obfuscated => self.obfuscated = Some(true),
            Code::Bold => self.bold = Some(true),
            Code::Italic => self.italic = Some(true),
            Code::StrikeThrough => self.strikethrough = Some(true),
            Code::Underlined => self.underlined = Some(true),
            _ => (),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StringComponent {
    #[serde(flatten)] pub base: Base,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TranslationComponent {
    #[serde(flatten)] pub base: Base,
    pub translate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub with: Option<Vec<ComponentWrapper>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Score {
    pub name: String,
    pub objective: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScoreComponent {
    #[serde(flatten)] pub base: Base,
    pub score: Score,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SelectorComponent {
    #[serde(flatten)] pub base: Base,
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

impl From<StringComponent> for Component {
    fn from(f: StringComponent) -> Self {
        Component::String(f)
    }
}

impl From<TranslationComponent> for Component {
    fn from(f: TranslationComponent) -> Self {
        Component::Translation(f)
    }
}

impl From<ScoreComponent> for Component {
    fn from(f: ScoreComponent) -> Self {
        Component::Score(f)
    }
}

impl From<SelectorComponent> for Component {
    fn from(f: SelectorComponent) -> Self {
        Component::Selector(f)
    }
}

impl Component {
    fn get_base(&mut self) -> &mut Base {
        match *self {
            Component::String(ref mut x) => &mut x.base,
            Component::Translation(ref mut x) => &mut x.base,
            Component::Score(ref mut x) => &mut x.base,
            Component::Selector(ref mut x) => &mut x.base,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ComponentWrapper(pub Component);

pub fn wrap<T: Into<Component>>(t: T) -> ComponentWrapper {
    ComponentWrapper(t.into())
}

impl<'de> Deserialize<'de> for ComponentWrapper {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ComponentWrapperVisitor;

        impl<'de> Visitor<'de> for ComponentWrapperVisitor {
            type Value = ComponentWrapper;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "object, array, or primitive")
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(ComponentWrapper(Component::String(StringComponent{text: v.to_string(), base: Default::default()})))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(ComponentWrapper(Component::String(StringComponent{text: v.to_string(), base: Default::default()})))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                Ok(ComponentWrapper(Component::String(StringComponent{text: v.to_string(), base: Default::default()})))
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(ComponentWrapper(Component::String(StringComponent{text: v.into(), base: Default::default()})))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let first = seq.next_element::<ComponentWrapper>()?;
                if first.is_none() {
                    return Err(de::Error::invalid_length(0, &"at least 1"));
                }
                let mut first = first.unwrap();
                first.0.get_base().extra = Some(Vec::new());

                while let Some(e) = seq.next_element()? {
                    first.0.get_base().extra.as_mut().unwrap().push(e);
                }

                Ok(first)
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                Ok(ComponentWrapper(Component::deserialize(de::value::MapAccessDeserializer::new(map))?))
            }
        }

        deserializer.deserialize_any(ComponentWrapperVisitor)
    }
}

#[derive(Debug, Serialize)]
pub struct Chat(pub ComponentWrapper);

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ChatVisitor;

        impl<'de> Visitor<'de> for ChatVisitor {
            type Value = Chat;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "object or array")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
                Ok(Chat(ComponentWrapper::deserialize(de::value::SeqAccessDeserializer::new(seq))?))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                Ok(Chat(ComponentWrapper::deserialize(de::value::MapAccessDeserializer::new(map))?))
            }
        }

        deserializer.deserialize_any(ChatVisitor)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IOError(err: io::Error) {
            description(err.description())
            display("io error: {}", err)
            cause(err)
            from()
        }
        JSONError(err: serde_json::Error) {
            description(err.description())
            display("json error: {}", err)
            cause(err)
            from()
        }
    }

}

impl Chat {
    pub fn write_proto<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        let s = serde_json::to_string(self).unwrap();
        proto::data::write_string(w, &s)
    }

    pub fn read_proto<R: io::BufRead>(r: &mut R) -> proto::Result<Chat> {
        let s = proto::data::read_string(r)?;
        let chat: Chat = serde_json::from_str(&s).map_err(|e| Error::from(e))?;
        Ok(chat)
    }
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
        assert_eq!(color, Color::Black);
    }

    #[test]
    #[should_panic]
    fn test_enum_de_wrong() {
        let st = "garbage";
        let _color: Color = serde_json::from_str(st).unwrap();
    }

    #[test]
    fn test_ser() {
        let com = Component::String(StringComponent {
            base: Base {
                bold: Some(true),
                ..Default::default()
            },
            text: "Hello".to_string(),
        });
        let st = serde_json::to_string(&com).unwrap();
        assert_eq!(st, "{\"bold\":true,\"text\":\"Hello\"}");
    }

    #[test]
    fn test_de() {
        let com: ComponentWrapper = serde_json::from_str("{\"bold\":true,\"text\":\"Hello\",\"extra\":[\"hi\"]}").unwrap();
        match com.0 {
            Component::String(s) => {
                assert_eq!(s.base.bold, Some(true));
                assert_eq!(s.text, "Hello");
                match s.base.extra.as_ref().unwrap()[0].0 {
                    Component::String(ref s) => assert_eq!(s.text, "hi"),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }

        let com: ComponentWrapper = serde_json::from_str("[1, 2, \"hi\"]").unwrap();
        match com.0 {
            Component::String(s) =>  { 
                assert_eq!(s.text, "1");
                let extra = s.base.extra.as_ref().unwrap();
                match extra[0].0 {
                    Component::String(ref s) => assert_eq!(s.text, "2"),
                    _ => panic!(),
                }
                match extra[1].0 {
                    Component::String(ref s) => assert_eq!(s.text, "hi"),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}
