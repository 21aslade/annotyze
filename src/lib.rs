use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

pub mod analytics;

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Clone, Copy, Debug)]
pub enum AnnotTy {
    Highlight,
    Reference,
    Journal,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Annotation {
    #[serde(rename = "type")]
    pub ty: AnnotTy,
    #[serde(deserialize_with = "de_undefined")]
    pub title: Option<String>,
    #[serde(deserialize_with = "de_empty_str", rename = "note text")]
    pub note_text: Option<String>,
    #[serde(deserialize_with = "de_undefined", rename = "source location")]
    pub url: Option<String>,
    pub tags: String,
    #[serde(rename = "study set")]
    pub study_set: String,
    #[serde(rename = "last updated")]
    pub last_updated: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

fn de_empty_str<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    Ok((!str.is_empty()).then(|| str))
}

fn de_undefined<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(deserializer)?;
    Ok((str != "undefined").then(|| str))
}
