use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct TagVariant {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct Tag {
    pub(crate) tag: String,
    pub(crate) variants: Vec<TagVariant>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct Frame {
    pub(crate) title: String,
    pub(crate) toggles: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct AnimateData {
    pub frames: Vec<Frame>,
    pub tags: Vec<String>,
    pub variants: Vec<TagVariant>,
    pub delay: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub(crate) struct DiagramMeta {
    pub title: Option<String>,
    pub animate: Option<AnimateData>,
    #[serde(rename = "animate-yml-file")]
    pub animate_yml_file: Option<String>,
}
