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
    pub story_prev: Option<Url>,
    pub next: Url,
    pub theme: Theme,
}

pub fn without_holes(page: usize) -> usize {
    let holes = [2399, 3038, 3088, 6370, 7902, 7903, 7904];
    let mut idx = holes
        .iter()
        .enumerate()
        .find(|&(_, &x)| page < x)
        .map(|x| x.0)
        .unwrap_or(holes.len());
    if page > 6245 && page < 7448 {
        idx -= 1;
    }
    page - idx
}

#[cfg(test)]
mod test {
    #[test]
    fn without_holes() {
        macro_rules! cases {
            ($($a:expr, $b:expr),*) => {
                $(assert_eq!(super::without_holes($a), $b, stringify!($a));)*
            };
        }
        cases!(
            1, 1, 2398, 2398, 2400, 2399, 3037, 3036, 3039, 3037, 3086, 3084, 3089, 3086, 6369,
            6367, 6371, 6368, 7901, 7897, 7905, 7898, 8128, 8121
        );
    }
}
