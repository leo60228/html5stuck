use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Pesterlog {
    pub typ: String,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Theme {
    Normal,
    Password,
    LinkInFlash,
    Combo { pesterlog: Option<Pesterlog> },
}

/// tavros struct
/// tavros struct
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Page {
    pub num: usize,
    pub title: Option<String>,
    pub contents: Option<String>,
    pub pesterlog: Option<Pesterlog>,
    pub story_next: Option<Url>,
    pub next: Url,
    pub theme: Theme,
}
