use proto;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json;
use std::convert::From;
use std::fmt;
use std::io;
use text::Code;

mod color;

pub use self::color::*;

mod events;

pub use self::events::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
    #[serde(skip_serializing_if = "Option::is_none")] pub extra: Option<Vec<Wrapper>>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StringComponent {
    #[serde(flatten)] pub base: Base,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranslationComponent {
    #[serde(flatten)] pub base: Base,
    pub translate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub with: Option<Vec<Wrapper>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Score {
    pub name: String,
    pub objective: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreComponent {
    #[serde(flatten)] pub base: Base,
    pub score: Score,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectorComponent {
    #[serde(flatten)] pub base: Base,
    pub selector: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Component {
    String(StringComponent),
    Translation(TranslationComponent),
    Score(ScoreComponent),
    Selector(SelectorComponent),
}

impl_from_for_newtype_enum!(Component::String, StringComponent);
impl_from_for_newtype_enum!(Component::Translation, TranslationComponent);
impl_from_for_newtype_enum!(Component::Score, ScoreComponent);
impl_from_for_newtype_enum!(Component::Selector, SelectorComponent);

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

#[derive(Debug, Serialize, Clone)]
pub struct Wrapper(pub Component);

impl Default for Wrapper {
    fn default() -> Self {
        Wrapper(Component::String(StringComponent::default()))
    }
}

impl<'de> Deserialize<'de> for Wrapper {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ComponentWrapperVisitor;

        impl<'de> Visitor<'de> for ComponentWrapperVisitor {
            type Value = Wrapper;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "object, array, or primitive")
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Wrapper(Component::from(StringComponent { text: v.to_string(), base: Default::default() })))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Wrapper(Component::from(StringComponent { text: v.to_string(), base: Default::default() })))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                Ok(Wrapper(Component::from(StringComponent { text: v.to_string(), base: Default::default() })))
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(Wrapper(Component::from(StringComponent { text: v.into(), base: Default::default() })))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let first = seq.next_element::<Wrapper>()?;
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
                Ok(Wrapper(Component::deserialize(de::value::MapAccessDeserializer::new(map))?))
            }
        }

        deserializer.deserialize_any(ComponentWrapperVisitor)
    }
}

#[derive(Debug, Serialize, Clone, Default)]
// box to reduce size.
pub struct Chat(pub Box<Wrapper>);

impl From<Component> for Chat {
    fn from(c: Component) -> Chat {
        Chat(Box::new(Wrapper(c)))
    }
}

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ChatVisitor;

        impl<'de> Visitor<'de> for ChatVisitor {
            type Value = Chat;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "object or array")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
                Ok(Chat(Box::new(Wrapper::deserialize(de::value::SeqAccessDeserializer::new(seq))?)))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                Ok(Chat(Box::new(Wrapper::deserialize(de::value::MapAccessDeserializer::new(map))?)))
            }
        }

        deserializer.deserialize_any(ChatVisitor)
    }
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "json error: {}", _0)]
    JSONError(#[cause] serde_json::Error),
}

impl_from_for_newtype_enum!(Error::JSONError, serde_json::Error);

impl Chat {
    pub fn write_proto<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        let s = serde_json::to_string(self).unwrap();
        proto::data::write_string(w, &s)
    }

    pub fn read_proto<R: io::BufRead>(r: &mut R) -> proto::Result<Chat> {
        let s = proto::data::read_string(r)?;
        let chat: Chat = serde_json::from_str(&s).map_err(Error::from)?;
        Ok(chat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let com = Component::from(StringComponent {
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
        let com: Wrapper = serde_json::from_str("{\"bold\":true,\"text\":\"Hello\",\"extra\":[\"hi\"]}").unwrap();
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

        let com: Wrapper = serde_json::from_str("[1, 2, \"hi\"]").unwrap();
        match com.0 {
            Component::String(s) => {
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
