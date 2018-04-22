#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClickAction {
    OpenUrl,
    RunCommand,
    SuggestCommand,
    ChangePage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClickEvent {
    pub action: ClickAction,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HoverAction {
    ShowText,
    ShowItem,
    ShowEntity,
    ShowAchievement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HoverEvent {
    pub action: HoverAction,
    pub value: String,
}